// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/asset-pipe/src/texture.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 4b281954bf3032df8465118ec2de0a7e5588536679690b0700f918b3a54a8c02
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: texture.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 1727, 3165, 3166, 3168, 3169, 3187, 4139, 4143, 4150
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.86"}]
// use asset; // namespace candidate
//   package identities: [{"name":"asset","source":"local_or_path","version":"0.1.0"}]
// use asset_pipe; // namespace candidate
//   package identities: [{"name":"asset-pipe","source":"local_or_path","version":"0.1.0"}]
// use bytemuck; // namespace candidate
//   package identities: [{"name":"bytemuck","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.16.0"}]
// use byteorder; // namespace candidate
//   package identities: [{"name":"byteorder","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.5.0"}]
// use core; // namespace candidate
//   package identities: []
// use ddsfile; // namespace candidate
//   package identities: [{"name":"ddsfile","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.1"}]
// use easy_parallel; // namespace candidate
//   package identities: [{"name":"easy-parallel","source":"registry+https://github.com/rust-lang/crates.io-index","version":"3.2.0"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use image; // namespace candidate
//   package identities: [{"name":"image","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.24.9"},{"name":"image","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.25.1"}]
// use intel_tex_2; // namespace candidate
//   package identities: [{"name":"intel_tex_2","source":"git+https://github.com/PounceLight/intel-tex-rs-2.git?rev=4fb0e0b70f868bccd529d45e1626de2f55a20b83#4fb0e0b70f868bccd529d45e1626de2f55a20b83","version":"0.4.0"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use relative_path; // namespace candidate
//   package identities: [{"name":"relative-path","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.9.0"}]
// use rkyv; // namespace candidate
//   package identities: [{"name":"rkyv","source":"git+https://github.com/PounceLight/rkyv.git?rev=031d58108ccf8c0e125b06b017f72b4afd67e9a3#031d58108ccf8c0e125b06b017f72b4afd67e9a3","version":"0.7.39"}]
// use ron; // namespace candidate
//   package identities: [{"name":"ron","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968","version":"0.8.1"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.200"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::compile_texture_asset
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1727,"record_offset":28952,"rva":24874832,"va":5393583952}
// 编译实例 RVAs: 0x17b8f50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-10942029","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393583952,"raw_hex":"e8b30959ff","size":5,"soft_fail":false,"target_va":5382644528,"va":5393586552,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404b1430
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19956216","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393583952,"raw_hex":"e8087ecffe","size":5,"soft_fail":false,"target_va":5373629488,"va":5393585699,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=7
//     target_aliases=92 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18692943","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393583952,"raw_hex":"e8b1c4e2fe","size":5,"soft_fail":false,"target_va":5374900736,"va":5393593674,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18505385","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393583952,"raw_hex":"e857a1e5fe","size":5,"soft_fail":false,"target_va":5375081488,"va":5393586868,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=32
//     target_aliases=1 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18502624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393583952,"raw_hex":"e820ace5fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5393584123,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=5
//     target_aliases=69 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18069610","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393583952,"raw_hex":"e89647ecfe","size":5,"soft_fail":false,"target_va":5375516032,"va":5393585637,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140889e60
//     owner=unresolved_namespace; count=5
//     target_aliases=3 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15922883","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393583952,"raw_hex":"e83d090dff","size":5,"soft_fail":false,"target_va":5377662560,"va":5393585438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d58e70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10883424","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393583952,"raw_hex":"e8a0ee59ff","size":5,"soft_fail":false,"target_va":5382704752,"va":5393588171,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d689e0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10816241","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393583952,"raw_hex":"e80ff55aff","size":5,"soft_fail":false,"target_va":5382769120,"va":5393585356,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=8
//     target_aliases=5 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10756415","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393583952,"raw_hex":"e8c1de5bff","size":5,"soft_fail":false,"target_va":5382827728,"va":5393584138,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417b52d0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26664","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393583952,"raw_hex":"e8d897ffff","size":5,"soft_fail":false,"target_va":5393568464,"va":5393595123,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420d7650
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9556023","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393583952,"raw_hex":"e837d09100","size":5,"soft_fail":false,"target_va":5403145808,"va":5393589780,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420d77e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9556543","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393583952,"raw_hex":"e83fd29100","size":5,"soft_fail":false,"target_va":5403146208,"va":5393589660,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420d7960
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9557849","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393583952,"raw_hex":"e859d79100","size":5,"soft_fail":false,"target_va":5403146592,"va":5393588738,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420d7ac0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9561562","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393583952,"raw_hex":"e8dae59100","size":5,"soft_fail":false,"target_va":5403146944,"va":5393585377,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420db6c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9571447","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393583952,"raw_hex":"e8770c9200","size":5,"soft_fail":false,"target_va":5403162304,"va":5393590852,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420e0930
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9598149","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393583952,"raw_hex":"e8c5749200","size":5,"soft_fail":false,"target_va":5403183408,"va":5393585254,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18262342","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393583952,"raw_hex":"e846a91601","size":5,"soft_fail":false,"target_va":5411847152,"va":5393584805,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18256565","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393583952,"raw_hex":"e8b5921601","size":5,"soft_fail":false,"target_va":5411847168,"va":5393590598,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t-10748784","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393583952,"raw_hex":"e890fc5bff","size":5,"soft_fail":false,"target_va":5382840208,"va":5393588987,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18203477","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393583952,"raw_hex":"e855c31501","size":5,"soft_fail":false,"target_va":5411791504,"va":5393588022,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t17948273","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393583952,"raw_hex":"e871de1101","size":5,"soft_fail":false,"target_va":5411540963,"va":5393592685,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::__private::format_err
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t18094973","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393583952,"raw_hex":"e87d1b1401","size":5,"soft_fail":false,"target_va":5411686944,"va":5393591966,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::construct::hd99151b32e25dfd8 (.llvm.2514920207462972046)
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t18199704","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393583952,"raw_hex":"e898b41501","size":5,"soft_fail":false,"target_va":5411788192,"va":5393588483,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::msg
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t18197703","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393583952,"raw_hex":"e8c7ac1501","size":5,"soft_fail":false,"target_va":5411786704,"va":5393588996,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..convert..From$LT$E$GT$$u20$for$u20$anyhow..Error$GT$::from
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t18198873","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393583952,"raw_hex":"e859b11501","size":5,"soft_fail":false,"target_va":5411789120,"va":5393590242,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] asset_pipe::texture::process_regular_image
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8719","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393583952,"raw_hex":"e80f220000","size":5,"soft_fail":false,"target_va":5393595408,"va":5393586684,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::iter::traits::iterator::Iterator::eq_by
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9596026","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393583952,"raw_hex":"e87a6c9200","size":5,"soft_fail":false,"target_va":5403181136,"va":5393585105,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t17948951","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393583952,"raw_hex":"e817e11101","size":5,"soft_fail":false,"target_va":5411541776,"va":5393592820,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t17886370","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393583952,"raw_hex":"e8a2ec1001","size":5,"soft_fail":false,"target_va":5411478980,"va":5393592605,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t17948565","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393583952,"raw_hex":"e895df1101","size":5,"soft_fail":false,"target_va":5411541904,"va":5393593334,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=9
//     evidence={"assembly":"\tcallq\t17950215","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393583952,"raw_hex":"e807e61101","size":5,"soft_fail":false,"target_va":5411542880,"va":5393592660,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ddsfile::Dds::get_d3d_format
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"ddsfile","version":"0.5.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9595937","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393583952,"raw_hex":"e8216c9200","size":5,"soft_fail":false,"target_va":5403187040,"va":5393591098,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ddsfile::Dds::get_data
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ddsfile","version":"0.5.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9597388","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393583952,"raw_hex":"e8cc719200","size":5,"soft_fail":false,"target_va":5403189040,"va":5393591647,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ddsfile::Dds::get_dxgi_format
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ddsfile","version":"0.5.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-16605","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393583952,"raw_hex":"e823bfffff","size":5,"soft_fail":false,"target_va":5393576592,"va":5393593192,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ddsfile::Dds::read
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ddsfile","version":"0.5.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9595412","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393583952,"raw_hex":"e8146a9200","size":5,"soft_fail":false,"target_va":5403185168,"va":5393589751,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] relative_path::RelativePath::to_path
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"relative-path","version":"1.9.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9596366","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393583952,"raw_hex":"e8ce6d9200","size":5,"soft_fail":false,"target_va":5403180464,"va":5393584093,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] relative_path::RelativePath::with_extension
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"relative-path","version":"1.9.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9596047","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393583952,"raw_hex":"e88f6c9200","size":5,"soft_fail":false,"target_va":5403180096,"va":5393584044,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rkyv::util::to_bytes<asset::texture::blob::TextureAssetBlob,256>
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rkyv","version":"0.7.39","source":"git+https://github.com/PounceLight/rkyv.git?rev=031d58108ccf8c0e125b06b017f72b4afd67e9a3#031d58108ccf8c0e125b06b017f72b4afd67e9a3"}]
//     evidence={"assembly":"\tcallq\t-20772","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393583952,"raw_hex":"e8dcaeffff","size":5,"soft_fail":false,"target_va":5393569216,"va":5393589983,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::backtrace::Backtrace::capture
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-10893730","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393583952,"raw_hex":"e85ec659ff","size":5,"soft_fail":false,"target_va":5382694656,"va":5393588381,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirBuilder::_create
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-10878767","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393583952,"raw_hex":"e8d1005aff","size":5,"soft_fail":false,"target_va":5382709584,"va":5393588346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::File::open
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-7897550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393583952,"raw_hex":"e8327e87ff","size":5,"soft_fail":false,"target_va":5385687760,"va":5393585305,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::Metadata::modified
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-10875472","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393583952,"raw_hex":"e8b00d5aff","size":5,"soft_fail":false,"target_va":5382708832,"va":5393584299,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::OpenOptions::_open
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-10880056","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393583952,"raw_hex":"e8c8fb59ff","size":5,"soft_fail":false,"target_va":5382708672,"va":5393588723,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-10884814","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393583952,"raw_hex":"e832e959ff","size":5,"soft_fail":false,"target_va":5382704816,"va":5393589625,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::io::Write::write_all
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t9593746","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393583952,"raw_hex":"e892639200","size":5,"soft_fail":false,"target_va":5403184176,"va":5393590425,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::components
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-10839564","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393583952,"raw_hex":"e8f4995aff","size":5,"soft_fail":false,"target_va":5382745088,"va":5393584647,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::extension
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-10844174","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393583952,"raw_hex":"e8f2875aff","size":5,"soft_fail":false,"target_va":5382743968,"va":5393588137,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::parent
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-10847117","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393583952,"raw_hex":"e8737c5aff","size":5,"soft_fail":false,"target_va":5382741200,"va":5393588312,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::stat
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-10807576","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393583952,"raw_hex":"e8e8165bff","size":5,"soft_fail":false,"target_va":5382776592,"va":5393584163,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5393583952,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5393584472,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1727,"record_offset":33852,"parent_record_offset":28952,"ancestor_record_offset":28952,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":1727,"record_offset":39764,"parent_record_offset":28952,"ancestor_record_offset":28952,"inlinee":8212,"source_line":46,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] asset::texture::desc::impl$27::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":1727,"record_offset":45760,"parent_record_offset":28952,"ancestor_record_offset":28952,"inlinee":77744,"source_line":36,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] asset::texture::desc::impl$31::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":1727,"record_offset":30812,"parent_record_offset":28952,"ancestor_record_offset":28952,"inlinee":77719,"source_line":36,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] asset_pipe::texture::process_dds_image
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":1727,"record_offset":40428,"parent_record_offset":28952,"ancestor_record_offset":28952,"inlinee":77742,"source_line":532,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] asset_pipe::texture::process_image_array
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":1727,"record_offset":33880,"parent_record_offset":28952,"ancestor_record_offset":28952,"inlinee":77729,"source_line":596,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] byteorder::io::ReadBytesExt::read_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"byteorder","version":"1.5.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/byteorder-1.5.0/src/io.rs
//     evidence={"module":1727,"record_offset":32904,"parent_record_offset":28952,"ancestor_record_offset":28952,"inlinee":21404,"source_line":217,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] byteorder::io::WriteBytesExt::write_u32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"byteorder","version":"1.5.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/byteorder-1.5.0/src/io.rs
//     evidence={"module":1727,"record_offset":45408,"parent_record_offset":28952,"ancestor_record_offset":28952,"inlinee":9439,"source_line":1259,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1727,"record_offset":33128,"parent_record_offset":28952,"ancestor_record_offset":28952,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1727,"record_offset":33160,"parent_record_offset":28952,"ancestor_record_offset":28952,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1727,"record_offset":39256,"parent_record_offset":28952,"ancestor_record_offset":28952,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1727,"record_offset":29176,"parent_record_offset":28952,"ancestor_record_offset":28952,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1727,"record_offset":29660,"parent_record_offset":28952,"ancestor_record_offset":28952,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1727,"record_offset":39472,"parent_record_offset":28952,"ancestor_record_offset":28952,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1727,"record_offset":32256,"parent_record_offset":28952,"ancestor_record_offset":28952,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1727,"record_offset":39356,"parent_record_offset":28952,"ancestor_record_offset":28952,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1727,"record_offset":33676,"parent_record_offset":28952,"ancestor_record_offset":28952,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1727,"record_offset":33044,"parent_record_offset":28952,"ancestor_record_offset":28952,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1727,"record_offset":39292,"parent_record_offset":28952,"ancestor_record_offset":28952,"inlinee":5178,"source_line":853,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] std::fs::File::create
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1727,"record_offset":39904,"parent_record_offset":28952,"ancestor_record_offset":28952,"inlinee":11592,"source_line":450,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] std::fs::create_dir_all
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1727,"record_offset":39680,"parent_record_offset":28952,"ancestor_record_offset":28952,"inlinee":8207,"source_line":2766,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::exists
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1727,"record_offset":29884,"parent_record_offset":28952,"ancestor_record_offset":28952,"inlinee":13003,"source_line":3025,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::metadata
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1727,"record_offset":29572,"parent_record_offset":28952,"ancestor_record_offset":28952,"inlinee":14362,"source_line":2913,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1727,"record_offset":29432,"parent_record_offset":28952,"ancestor_record_offset":28952,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] std::time::impl$29::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\time.rs
//     evidence={"module":1727,"record_offset":32132,"parent_record_offset":28952,"ancestor_record_offset":28952,"inlinee":16252,"source_line":242,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1727,"record_offset":34968,"parent_record_offset":34944,"ancestor_record_offset":28952,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1727,"record_offset":34992,"parent_record_offset":34968,"ancestor_record_offset":28952,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1727,"record_offset":29376,"parent_record_offset":29352,"ancestor_record_offset":28952,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1727,"record_offset":34944,"parent_record_offset":34904,"ancestor_record_offset":28952,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1727,"record_offset":29352,"parent_record_offset":29296,"ancestor_record_offset":28952,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1727,"record_offset":30316,"parent_record_offset":30288,"ancestor_record_offset":28952,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1727,"record_offset":37320,"parent_record_offset":33880,"ancestor_record_offset":28952,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1727,"record_offset":37368,"parent_record_offset":37344,"ancestor_record_offset":28952,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1727,"record_offset":36140,"parent_record_offset":36112,"ancestor_record_offset":28952,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1727,"record_offset":34824,"parent_record_offset":34784,"ancestor_record_offset":28952,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1727,"record_offset":29324,"parent_record_offset":29296,"ancestor_record_offset":28952,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1727,"record_offset":29296,"parent_record_offset":29272,"ancestor_record_offset":28952,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1727,"record_offset":36200,"parent_record_offset":36168,"ancestor_record_offset":28952,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1727,"record_offset":36168,"parent_record_offset":36140,"ancestor_record_offset":28952,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1727,"record_offset":34904,"parent_record_offset":34864,"ancestor_record_offset":28952,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1727,"record_offset":34864,"parent_record_offset":34824,"ancestor_record_offset":28952,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1727,"record_offset":29272,"parent_record_offset":29248,"ancestor_record_offset":28952,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1727,"record_offset":43060,"parent_record_offset":43008,"ancestor_record_offset":28952,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1727,"record_offset":42824,"parent_record_offset":42780,"ancestor_record_offset":28952,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1727,"record_offset":42780,"parent_record_offset":42736,"ancestor_record_offset":28952,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1727,"record_offset":42692,"parent_record_offset":42648,"ancestor_record_offset":28952,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1727,"record_offset":42736,"parent_record_offset":42692,"ancestor_record_offset":28952,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1727,"record_offset":42648,"parent_record_offset":40428,"ancestor_record_offset":28952,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1727,"record_offset":46052,"parent_record_offset":46004,"ancestor_record_offset":28952,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1727,"record_offset":35088,"parent_record_offset":35048,"ancestor_record_offset":28952,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1727,"record_offset":36112,"parent_record_offset":36064,"ancestor_record_offset":28952,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1727,"record_offset":34744,"parent_record_offset":34684,"ancestor_record_offset":28952,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1727,"record_offset":34784,"parent_record_offset":34744,"ancestor_record_offset":28952,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1727,"record_offset":45964,"parent_record_offset":45928,"ancestor_record_offset":28952,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1727,"record_offset":34588,"parent_record_offset":34540,"ancestor_record_offset":28952,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1727,"record_offset":35992,"parent_record_offset":33880,"ancestor_record_offset":28952,"inlinee":4917,"source_line":3492,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1727,"record_offset":35372,"parent_record_offset":35324,"ancestor_record_offset":28952,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1727,"record_offset":46004,"parent_record_offset":45964,"ancestor_record_offset":28952,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\partial_eq.rs
//     evidence={"module":1727,"record_offset":32072,"parent_record_offset":30812,"ancestor_record_offset":28952,"inlinee":5608,"source_line":15,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1727,"record_offset":35484,"parent_record_offset":35372,"ancestor_record_offset":28952,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1727,"record_offset":36892,"parent_record_offset":36064,"ancestor_record_offset":28952,"inlinee":6531,"source_line":13,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1727,"record_offset":36832,"parent_record_offset":36804,"ancestor_record_offset":28952,"inlinee":6529,"source_line":30,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1727,"record_offset":35048,"parent_record_offset":34684,"ancestor_record_offset":28952,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":1727,"record_offset":34636,"parent_record_offset":34588,"ancestor_record_offset":28952,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":1727,"record_offset":34684,"parent_record_offset":34636,"ancestor_record_offset":28952,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":1727,"record_offset":39836,"parent_record_offset":39796,"ancestor_record_offset":28952,"inlinee":7901,"source_line":173,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":1727,"record_offset":39796,"parent_record_offset":39764,"ancestor_record_offset":28952,"inlinee":7918,"source_line":23,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::blob::impl$3::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/blob.rs
//     evidence={"module":1727,"record_offset":35704,"parent_record_offset":35680,"ancestor_record_offset":28952,"inlinee":77737,"source_line":5,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::blob::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/blob.rs
//     evidence={"module":1727,"record_offset":35760,"parent_record_offset":35736,"ancestor_record_offset":28952,"inlinee":77739,"source_line":196,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$13::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":1727,"record_offset":31952,"parent_record_offset":31800,"ancestor_record_offset":28952,"inlinee":77725,"source_line":11,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$20::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":1727,"record_offset":32040,"parent_record_offset":31800,"ancestor_record_offset":28952,"inlinee":77727,"source_line":19,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$26::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":1727,"record_offset":31800,"parent_record_offset":30812,"ancestor_record_offset":28952,"inlinee":77721,"source_line":27,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$3::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":1727,"record_offset":31924,"parent_record_offset":31800,"ancestor_record_offset":28952,"inlinee":77723,"source_line":4,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture::process_dds_image::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":1727,"record_offset":41728,"parent_record_offset":41696,"ancestor_record_offset":28952,"inlinee":77749,"source_line":544,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture::process_dds_image::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":1727,"record_offset":41840,"parent_record_offset":41696,"ancestor_record_offset":28952,"inlinee":77750,"source_line":545,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] byteorder::impl$3::write_u32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"byteorder","version":"1.5.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/byteorder-1.5.0/src/lib.rs
//     evidence={"module":1727,"record_offset":45436,"parent_record_offset":45408,"ancestor_record_offset":28952,"inlinee":9451,"source_line":2169,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1727,"record_offset":30348,"parent_record_offset":30316,"ancestor_record_offset":28952,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1727,"record_offset":43084,"parent_record_offset":43060,"ancestor_record_offset":28952,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1727,"record_offset":43108,"parent_record_offset":43084,"ancestor_record_offset":28952,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\equality.rs
//     evidence={"module":1727,"record_offset":31864,"parent_record_offset":31836,"ancestor_record_offset":28952,"inlinee":10090,"source_line":9,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\equality.rs
//     evidence={"module":1727,"record_offset":35596,"parent_record_offset":33880,"ancestor_record_offset":28952,"inlinee":14532,"source_line":13,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\equality.rs
//     evidence={"module":1727,"record_offset":31888,"parent_record_offset":31864,"ancestor_record_offset":28952,"inlinee":9478,"source_line":146,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\equality.rs
//     evidence={"module":1727,"record_offset":35620,"parent_record_offset":35596,"ancestor_record_offset":28952,"inlinee":9488,"source_line":152,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1727,"record_offset":35680,"parent_record_offset":33880,"ancestor_record_offset":28952,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1727,"record_offset":41520,"parent_record_offset":40428,"ancestor_record_offset":28952,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1727,"record_offset":32004,"parent_record_offset":31980,"ancestor_record_offset":28952,"inlinee":5188,"source_line":1740,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1727,"record_offset":31420,"parent_record_offset":31124,"ancestor_record_offset":28952,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1727,"record_offset":40052,"parent_record_offset":39904,"ancestor_record_offset":28952,"inlinee":5126,"source_line":703,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$6::try_into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1727,"record_offset":42556,"parent_record_offset":40428,"ancestor_record_offset":28952,"inlinee":9618,"source_line":799,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$118::try_from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\num.rs
//     evidence={"module":1727,"record_offset":42580,"parent_record_offset":42556,"ancestor_record_offset":28952,"inlinee":12127,"source_line":255,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1727,"record_offset":42528,"parent_record_offset":40428,"ancestor_record_offset":28952,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1727,"record_offset":44380,"parent_record_offset":40428,"ancestor_record_offset":28952,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1727,"record_offset":33000,"parent_record_offset":32976,"ancestor_record_offset":28952,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1727,"record_offset":43156,"parent_record_offset":43132,"ancestor_record_offset":28952,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::copy_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":1727,"record_offset":35276,"parent_record_offset":35208,"ancestor_record_offset":28952,"inlinee":9582,"source_line":30,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":1727,"record_offset":35168,"parent_record_offset":35128,"ancestor_record_offset":28952,"inlinee":9580,"source_line":71,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1727,"record_offset":34200,"parent_record_offset":33880,"ancestor_record_offset":28952,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::Rev::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\rev.rs
//     evidence={"module":1727,"record_offset":31644,"parent_record_offset":31504,"ancestor_record_offset":28952,"inlinee":10401,"source_line":20,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1727,"record_offset":34540,"parent_record_offset":33880,"ancestor_record_offset":28952,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1727,"record_offset":35128,"parent_record_offset":35088,"ancestor_record_offset":28952,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1727,"record_offset":35324,"parent_record_offset":35276,"ancestor_record_offset":28952,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1727,"record_offset":30372,"parent_record_offset":30348,"ancestor_record_offset":28952,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1727,"record_offset":43132,"parent_record_offset":43108,"ancestor_record_offset":28952,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1727,"record_offset":36692,"parent_record_offset":36340,"ancestor_record_offset":28952,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1727,"record_offset":36224,"parent_record_offset":36200,"ancestor_record_offset":28952,"inlinee":5005,"source_line":2021,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1727,"record_offset":31256,"parent_record_offset":31220,"ancestor_record_offset":28952,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1727,"record_offset":31220,"parent_record_offset":31192,"ancestor_record_offset":28952,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1727,"record_offset":37344,"parent_record_offset":37320,"ancestor_record_offset":28952,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1727,"record_offset":37284,"parent_record_offset":33880,"ancestor_record_offset":28952,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1727,"record_offset":41576,"parent_record_offset":41552,"ancestor_record_offset":28952,"inlinee":4666,"source_line":998,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1727,"record_offset":31836,"parent_record_offset":31800,"ancestor_record_offset":28952,"inlinee":5835,"source_line":2186,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1727,"record_offset":31556,"parent_record_offset":31504,"ancestor_record_offset":28952,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1727,"record_offset":33732,"parent_record_offset":33700,"ancestor_record_offset":28952,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1727,"record_offset":33760,"parent_record_offset":33700,"ancestor_record_offset":28952,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1727,"record_offset":34384,"parent_record_offset":34360,"ancestor_record_offset":28952,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1727,"record_offset":43300,"parent_record_offset":42824,"ancestor_record_offset":28952,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=110
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1727,"record_offset":29200,"parent_record_offset":29176,"ancestor_record_offset":28952,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1727,"record_offset":34164,"parent_record_offset":34140,"ancestor_record_offset":28952,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1727,"record_offset":30116,"parent_record_offset":30088,"ancestor_record_offset":28952,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1727,"record_offset":30164,"parent_record_offset":30140,"ancestor_record_offset":28952,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1727,"record_offset":30140,"parent_record_offset":30116,"ancestor_record_offset":28952,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1727,"record_offset":35528,"parent_record_offset":35208,"ancestor_record_offset":28952,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1727,"record_offset":34264,"parent_record_offset":34228,"ancestor_record_offset":28952,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1727,"record_offset":35432,"parent_record_offset":35372,"ancestor_record_offset":28952,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1727,"record_offset":29620,"parent_record_offset":29596,"ancestor_record_offset":28952,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1727,"record_offset":34420,"parent_record_offset":33880,"ancestor_record_offset":28952,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1727,"record_offset":34304,"parent_record_offset":33880,"ancestor_record_offset":28952,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1727,"record_offset":38372,"parent_record_offset":33880,"ancestor_record_offset":28952,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1727,"record_offset":31444,"parent_record_offset":31420,"ancestor_record_offset":28952,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1727,"record_offset":31468,"parent_record_offset":31444,"ancestor_record_offset":28952,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1727,"record_offset":32976,"parent_record_offset":32952,"ancestor_record_offset":28952,"inlinee":5246,"source_line":3720,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1727,"record_offset":34116,"parent_record_offset":33880,"ancestor_record_offset":28952,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1727,"record_offset":34140,"parent_record_offset":34116,"ancestor_record_offset":28952,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1727,"record_offset":35208,"parent_record_offset":35168,"ancestor_record_offset":28952,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1727,"record_offset":34228,"parent_record_offset":34200,"ancestor_record_offset":28952,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1727,"record_offset":33068,"parent_record_offset":33044,"ancestor_record_offset":28952,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1727,"record_offset":33092,"parent_record_offset":33068,"ancestor_record_offset":28952,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] ddsfile::Dds::get_depth
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ddsfile","version":"0.5.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ddsfile-0.5.1/src/lib.rs
//     evidence={"module":1727,"record_offset":41552,"parent_record_offset":40428,"ancestor_record_offset":28952,"inlinee":77746,"source_line":249,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] ddsfile::Dds::get_dxgi_format
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ddsfile","version":"0.5.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ddsfile-0.5.1/src/lib.rs
//     evidence={"module":1727,"record_offset":41608,"parent_record_offset":40428,"ancestor_record_offset":28952,"inlinee":77747,"source_line":218,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] ddsfile::format::dxgi::DxgiFormat::try_from_pixel_format
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ddsfile","version":"0.5.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ddsfile-0.5.1/src/format/dxgi.rs
//     evidence={"module":1727,"record_offset":41644,"parent_record_offset":41608,"ancestor_record_offset":28952,"inlinee":77709,"source_line":416,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1727,"record_offset":33788,"parent_record_offset":33700,"ancestor_record_offset":28952,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1727,"record_offset":41956,"parent_record_offset":40428,"ancestor_record_offset":28952,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1727,"record_offset":33700,"parent_record_offset":33676,"ancestor_record_offset":28952,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1727,"record_offset":33812,"parent_record_offset":33788,"ancestor_record_offset":28952,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1727,"record_offset":41436,"parent_record_offset":40428,"ancestor_record_offset":28952,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::as_ptr
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.7.39","source":"git+https://github.com/PounceLight/rkyv.git?rev=031d58108ccf8c0e125b06b017f72b4afd67e9a3#031d58108ccf8c0e125b06b017f72b4afd67e9a3"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-79c0f5fde0331f7b/031d581/rkyv/src/rel_ptr/mod.rs
//     evidence={"module":1727,"record_offset":35892,"parent_record_offset":35868,"ancestor_record_offset":28952,"inlinee":28664,"source_line":212,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::offset
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.7.39","source":"git+https://github.com/PounceLight/rkyv.git?rev=031d58108ccf8c0e125b06b017f72b4afd67e9a3#031d58108ccf8c0e125b06b017f72b4afd67e9a3"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-79c0f5fde0331f7b/031d581/rkyv/src/rel_ptr/mod.rs
//     evidence={"module":1727,"record_offset":35916,"parent_record_offset":35892,"ancestor_record_offset":28952,"inlinee":28665,"source_line":200,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RelPtr::as_ptr
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.7.39","source":"git+https://github.com/PounceLight/rkyv.git?rev=031d58108ccf8c0e125b06b017f72b4afd67e9a3#031d58108ccf8c0e125b06b017f72b4afd67e9a3"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-79c0f5fde0331f7b/031d581/rkyv/src/rel_ptr/mod.rs
//     evidence={"module":1727,"record_offset":35868,"parent_record_offset":35840,"ancestor_record_offset":28952,"inlinee":28662,"source_line":418,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::impl$18::to_isize
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.7.39","source":"git+https://github.com/PounceLight/rkyv.git?rev=031d58108ccf8c0e125b06b017f72b4afd67e9a3#031d58108ccf8c0e125b06b017f72b4afd67e9a3"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-79c0f5fde0331f7b/031d581/rkyv/src/rel_ptr/mod.rs
//     evidence={"module":1727,"record_offset":35940,"parent_record_offset":35916,"ancestor_record_offset":28952,"inlinee":28667,"source_line":110,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::util::aligned_vec::impl$0::drop
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"rkyv","version":"0.7.39","source":"git+https://github.com/PounceLight/rkyv.git?rev=031d58108ccf8c0e125b06b017f72b4afd67e9a3#031d58108ccf8c0e125b06b017f72b4afd67e9a3"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-79c0f5fde0331f7b/031d581/rkyv/src/util/aligned_vec.rs
//     evidence={"module":1727,"record_offset":37216,"parent_record_offset":37188,"ancestor_record_offset":28952,"inlinee":77741,"source_line":22,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::util::archived_root
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.7.39","source":"git+https://github.com/PounceLight/rkyv.git?rev=031d58108ccf8c0e125b06b017f72b4afd67e9a3#031d58108ccf8c0e125b06b017f72b4afd67e9a3"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-79c0f5fde0331f7b/031d581/rkyv/src/util/mod.rs
//     evidence={"module":1727,"record_offset":34336,"parent_record_offset":33880,"ancestor_record_offset":28952,"inlinee":77731,"source_line":147,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::util::archived_value
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.7.39","source":"git+https://github.com/PounceLight/rkyv.git?rev=031d58108ccf8c0e125b06b017f72b4afd67e9a3#031d58108ccf8c0e125b06b017f72b4afd67e9a3"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-79c0f5fde0331f7b/031d581/rkyv/src/util/mod.rs
//     evidence={"module":1727,"record_offset":34360,"parent_record_offset":34336,"ancestor_record_offset":28952,"inlinee":77732,"source_line":63,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::ArchivedVec::as_ptr
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.7.39","source":"git+https://github.com/PounceLight/rkyv.git?rev=031d58108ccf8c0e125b06b017f72b4afd67e9a3#031d58108ccf8c0e125b06b017f72b4afd67e9a3"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-79c0f5fde0331f7b/031d581/rkyv/src/vec/mod.rs
//     evidence={"module":1727,"record_offset":35840,"parent_record_offset":35816,"ancestor_record_offset":28952,"inlinee":28660,"source_line":32,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::ArchivedVec::as_slice
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.7.39","source":"git+https://github.com/PounceLight/rkyv.git?rev=031d58108ccf8c0e125b06b017f72b4afd67e9a3#031d58108ccf8c0e125b06b017f72b4afd67e9a3"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-79c0f5fde0331f7b/031d581/rkyv/src/vec/mod.rs
//     evidence={"module":1727,"record_offset":34480,"parent_record_offset":34456,"ancestor_record_offset":28952,"inlinee":28659,"source_line":50,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::ArchivedVec::len
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.7.39","source":"git+https://github.com/PounceLight/rkyv.git?rev=031d58108ccf8c0e125b06b017f72b4afd67e9a3#031d58108ccf8c0e125b06b017f72b4afd67e9a3"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-79c0f5fde0331f7b/031d581/rkyv/src/vec/mod.rs
//     evidence={"module":1727,"record_offset":34504,"parent_record_offset":34480,"ancestor_record_offset":28952,"inlinee":77735,"source_line":38,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::impl$4::deref
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.7.39","source":"git+https://github.com/PounceLight/rkyv.git?rev=031d58108ccf8c0e125b06b017f72b4afd67e9a3#031d58108ccf8c0e125b06b017f72b4afd67e9a3"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-79c0f5fde0331f7b/031d581/rkyv/src/vec/mod.rs
//     evidence={"module":1727,"record_offset":34456,"parent_record_offset":33880,"ancestor_record_offset":28952,"inlinee":77734,"source_line":207,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1727,"record_offset":45852,"parent_record_offset":45812,"ancestor_record_offset":28952,"inlinee":8126,"source_line":642,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1727,"record_offset":29480,"parent_record_offset":29456,"ancestor_record_offset":28952,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1727,"record_offset":29528,"parent_record_offset":29504,"ancestor_record_offset":28952,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1727,"record_offset":29504,"parent_record_offset":29480,"ancestor_record_offset":28952,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1727,"record_offset":39732,"parent_record_offset":39680,"ancestor_record_offset":28952,"inlinee":8210,"source_line":3032,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1727,"record_offset":39704,"parent_record_offset":39680,"ancestor_record_offset":28952,"inlinee":8209,"source_line":3008,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1727,"record_offset":39928,"parent_record_offset":39904,"ancestor_record_offset":28952,"inlinee":11594,"source_line":1342,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1727,"record_offset":40248,"parent_record_offset":39904,"ancestor_record_offset":28952,"inlinee":11599,"source_line":1559,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::metadata
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1727,"record_offset":29596,"parent_record_offset":29572,"ancestor_record_offset":28952,"inlinee":12458,"source_line":2381,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1727,"record_offset":40656,"parent_record_offset":40428,"ancestor_record_offset":28952,"inlinee":12456,"source_line":267,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::Cursor::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\cursor.rs
//     evidence={"module":1727,"record_offset":40912,"parent_record_offset":40428,"ancestor_record_offset":28952,"inlinee":21628,"source_line":98,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::impl$5::read_exact
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\cursor.rs
//     evidence={"module":1727,"record_offset":32928,"parent_record_offset":32904,"ancestor_record_offset":28952,"inlinee":21967,"source_line":360,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":1727,"record_offset":30088,"parent_record_offset":30060,"ancestor_record_offset":28952,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":1727,"record_offset":30060,"parent_record_offset":30032,"ancestor_record_offset":28952,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$8::read_exact
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":1727,"record_offset":32952,"parent_record_offset":32928,"ancestor_record_offset":28952,"inlinee":20267,"source_line":347,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\os\windows\io\handle.rs
//     evidence={"module":1727,"record_offset":32396,"parent_record_offset":32368,"ancestor_record_offset":28952,"inlinee":5147,"source_line":386,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Components::prefix_verbatim
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1727,"record_offset":31192,"parent_record_offset":31124,"ancestor_record_offset":28952,"inlinee":10391,"source_line":646,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1727,"record_offset":29456,"parent_record_offset":29432,"ancestor_record_offset":28952,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Prefix::is_verbatim
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1727,"record_offset":31284,"parent_record_offset":31256,"ancestor_record_offset":28952,"inlinee":10393,"source_line":222,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$100::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1727,"record_offset":31164,"parent_record_offset":31124,"ancestor_record_offset":28952,"inlinee":10389,"source_line":371,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$116::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1727,"record_offset":31504,"parent_record_offset":31124,"ancestor_record_offset":28952,"inlinee":10395,"source_line":589,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$23::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1727,"record_offset":31124,"parent_record_offset":30844,"ancestor_record_offset":28952,"inlinee":10387,"source_line":987,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1727,"record_offset":45812,"parent_record_offset":45760,"ancestor_record_offset":28952,"inlinee":8124,"source_line":1738,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1727,"record_offset":30868,"parent_record_offset":30844,"ancestor_record_offset":28952,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$60::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1727,"record_offset":30844,"parent_record_offset":30812,"ancestor_record_offset":28952,"inlinee":10524,"source_line":2057,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1727,"record_offset":40076,"parent_record_offset":40052,"ancestor_record_offset":28952,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$89::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1727,"record_offset":31584,"parent_record_offset":31556,"ancestor_record_offset":28952,"inlinee":10397,"source_line":135,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$98::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1727,"record_offset":31616,"parent_record_offset":31504,"ancestor_record_offset":28952,"inlinee":10399,"source_line":371,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":1727,"record_offset":45892,"parent_record_offset":45852,"ancestor_record_offset":28952,"inlinee":8128,"source_line":13,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::create
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":1727,"record_offset":39992,"parent_record_offset":39928,"ancestor_record_offset":28952,"inlinee":11597,"source_line":218,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::truncate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":1727,"record_offset":40020,"parent_record_offset":39928,"ancestor_record_offset":28952,"inlinee":11598,"source_line":215,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":1727,"record_offset":39964,"parent_record_offset":39928,"ancestor_record_offset":28952,"inlinee":11596,"source_line":209,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::time::SystemTime::intervals
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\time.rs
//     evidence={"module":1727,"record_offset":32180,"parent_record_offset":32156,"ancestor_record_offset":28952,"inlinee":28246,"source_line":84,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::time::impl$2::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\time.rs
//     evidence={"module":1727,"record_offset":32156,"parent_record_offset":32132,"ancestor_record_offset":28952,"inlinee":16254,"source_line":110,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::sys_common::wtf8::impl$35::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys_common\wtf8.rs
//     evidence={"module":1727,"record_offset":45928,"parent_record_offset":45892,"ancestor_record_offset":28952,"inlinee":8130,"source_line":132,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 38, 47
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A65A
// 首条位置: {"module":3166,"inlinee":108122}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ron::de::tag::impl$1::deserialize_struct
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":4139,"record_offset":17448,"parent_record_offset":17424,"ancestor_record_offset":17424,"inlinee":5563,"source_line":221,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::_::impl$0::deserialize::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 38, 47
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A6DD
// 首条位置: {"module":4139,"inlinee":108253}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4139,"record_offset":32584,"parent_record_offset":32556,"ancestor_record_offset":32556,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":4139,"record_offset":32608,"parent_record_offset":32584,"ancestor_record_offset":32556,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":4139,"record_offset":32632,"parent_record_offset":32608,"ancestor_record_offset":32556,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":4139,"record_offset":32656,"parent_record_offset":32632,"ancestor_record_offset":32556,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 38, 47
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1EA2B
// 首条位置: {"module":4139,"inlinee":125483}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ron::de::impl$2::deserialize_identifier
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4139,"record_offset":32448,"parent_record_offset":32416,"ancestor_record_offset":32416,"inlinee":13285,"source_line":747,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":4139,"record_offset":32556,"parent_record_offset":32528,"ancestor_record_offset":32416,"inlinee":108253,"source_line":38,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":4139,"record_offset":32608,"parent_record_offset":32584,"ancestor_record_offset":32416,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":32492,"parent_record_offset":32448,"ancestor_record_offset":32416,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":4139,"record_offset":32632,"parent_record_offset":32608,"ancestor_record_offset":32416,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":4139,"record_offset":32656,"parent_record_offset":32632,"ancestor_record_offset":32416,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4139,"record_offset":32584,"parent_record_offset":32556,"ancestor_record_offset":32416,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4139,"record_offset":32528,"parent_record_offset":32448,"ancestor_record_offset":32416,"inlinee":5165,"source_line":1504,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::_::impl$0::deserialize::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 38, 47
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1EA29
// 首条位置: {"module":4139,"inlinee":125481}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4139,"record_offset":32968,"parent_record_offset":31756,"ancestor_record_offset":31756,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4139,"record_offset":34044,"parent_record_offset":31756,"ancestor_record_offset":31756,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":32868,"parent_record_offset":31756,"ancestor_record_offset":31756,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":34072,"parent_record_offset":31756,"ancestor_record_offset":31756,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4139,"record_offset":31812,"parent_record_offset":31756,"ancestor_record_offset":31756,"inlinee":6600,"source_line":1846,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4139,"record_offset":32904,"parent_record_offset":31756,"ancestor_record_offset":31756,"inlinee":6605,"source_line":1863,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":4139,"record_offset":33816,"parent_record_offset":33792,"ancestor_record_offset":31756,"inlinee":108179,"source_line":4,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":4139,"record_offset":32556,"parent_record_offset":32528,"ancestor_record_offset":31756,"inlinee":108253,"source_line":38,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":4139,"record_offset":32136,"parent_record_offset":32112,"ancestor_record_offset":31756,"inlinee":125483,"source_line":38,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":4139,"record_offset":32608,"parent_record_offset":32584,"ancestor_record_offset":31756,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4139,"record_offset":32040,"parent_record_offset":31848,"ancestor_record_offset":31756,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4139,"record_offset":32168,"parent_record_offset":31848,"ancestor_record_offset":31756,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4139,"record_offset":33332,"parent_record_offset":33244,"ancestor_record_offset":31756,"inlinee":5145,"source_line":1277,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4139,"record_offset":36952,"parent_record_offset":36552,"ancestor_record_offset":35432,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":32004,"parent_record_offset":31848,"ancestor_record_offset":31756,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":4139,"record_offset":32632,"parent_record_offset":32608,"ancestor_record_offset":31756,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":4139,"record_offset":32656,"parent_record_offset":32632,"ancestor_record_offset":31756,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4139,"record_offset":32584,"parent_record_offset":32556,"ancestor_record_offset":31756,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":4139,"record_offset":37024,"parent_record_offset":37000,"ancestor_record_offset":35432,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::id::Deserializer::new
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/id.rs
//     evidence={"module":4139,"record_offset":32072,"parent_record_offset":31848,"ancestor_record_offset":31756,"inlinee":13309,"source_line":11,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_identifier
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4139,"record_offset":32448,"parent_record_offset":32416,"ancestor_record_offset":31756,"inlinee":13285,"source_line":747,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$6::next_key_seed
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4139,"record_offset":31848,"parent_record_offset":31812,"ancestor_record_offset":31756,"inlinee":13307,"source_line":838,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$6::next_value_seed
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4139,"record_offset":33040,"parent_record_offset":33000,"ancestor_record_offset":31756,"inlinee":13322,"source_line":857,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_map
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":4139,"record_offset":36784,"parent_record_offset":36760,"ancestor_record_offset":35432,"inlinee":100077,"source_line":214,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_struct
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":4139,"record_offset":33304,"parent_record_offset":33244,"ancestor_record_offset":31756,"inlinee":5563,"source_line":221,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4139,"record_offset":32528,"parent_record_offset":32448,"ancestor_record_offset":31756,"inlinee":5165,"source_line":1504,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4139,"record_offset":32112,"parent_record_offset":31848,"ancestor_record_offset":31756,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$204::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4139,"record_offset":36760,"parent_record_offset":36736,"ancestor_record_offset":35432,"inlinee":73646,"source_line":1505,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$36::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4139,"record_offset":33244,"parent_record_offset":33212,"ancestor_record_offset":31756,"inlinee":125485,"source_line":2276,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$36::deserialize::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4139,"record_offset":33364,"parent_record_offset":33332,"ancestor_record_offset":31756,"inlinee":125487,"source_line":2414,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::_::impl$0::serialize<ref_mut$<ron::ser::Serializer<ref_mut$<alloc::string::String> > > >
// 记录形式: compiled_procedure; instances=2
// 源码行: 38, 47
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3187,"record_offset":8768,"rva":34485216,"va":5403194336}
// 编译实例 RVAs: 0x20e33e0, 0x20e3560
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1424520e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3599488","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403194336,"raw_hex":"e880ec3600","size":5,"soft_fail":false,"target_va":5406793952,"va":5403194459,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142452c40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3602012","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403194720,"raw_hex":"e85cf63600","size":5,"soft_fail":false,"target_va":5406796864,"va":5403194847,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1424552b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3611801","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403194720,"raw_hex":"e8991c3700","size":5,"soft_fail":false,"target_va":5406806704,"va":5403194898,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142455590
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3612936","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403194336,"raw_hex":"e808213700","size":5,"soft_fail":false,"target_va":5406807440,"va":5403194499,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142455e10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3615211","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403194336,"raw_hex":"e8eb293700","size":5,"soft_fail":false,"target_va":5406809616,"va":5403194400,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3187,"record_offset":9100,"parent_record_offset":8768,"ancestor_record_offset":8768,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3187,"record_offset":8972,"parent_record_offset":8768,"ancestor_record_offset":8768,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3187,"record_offset":9044,"parent_record_offset":8768,"ancestor_record_offset":8768,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3187,"record_offset":9152,"parent_record_offset":9124,"ancestor_record_offset":8768,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] ron::ser::impl$5::drop
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/ser/mod.rs
//     evidence={"module":3187,"record_offset":9124,"parent_record_offset":9100,"ancestor_record_offset":8768,"inlinee":5557,"source_line":1098,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::impl$1::default
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 38
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12F91
// 首条位置: {"module":1727,"inlinee":77713}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1727,"record_offset":15144,"parent_record_offset":15112,"ancestor_record_offset":15112,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1727,"record_offset":15692,"parent_record_offset":15660,"ancestor_record_offset":15112,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1727,"record_offset":15752,"parent_record_offset":15720,"ancestor_record_offset":15112,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1727,"record_offset":15548,"parent_record_offset":15524,"ancestor_record_offset":15112,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1727,"record_offset":15720,"parent_record_offset":15660,"ancestor_record_offset":15112,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1727,"record_offset":15296,"parent_record_offset":15256,"ancestor_record_offset":15112,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1727,"record_offset":15412,"parent_record_offset":15384,"ancestor_record_offset":15112,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1727,"record_offset":15500,"parent_record_offset":15384,"ancestor_record_offset":15112,"inlinee":7403,"source_line":1581,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1727,"record_offset":15576,"parent_record_offset":15548,"ancestor_record_offset":15112,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1727,"record_offset":15524,"parent_record_offset":15500,"ancestor_record_offset":15112,"inlinee":7404,"source_line":1212,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1727,"record_offset":15604,"parent_record_offset":15548,"ancestor_record_offset":15112,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1727,"record_offset":15200,"parent_record_offset":15172,"ancestor_record_offset":15112,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__init
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1727,"record_offset":15460,"parent_record_offset":15436,"ancestor_record_offset":15112,"inlinee":7402,"source_line":83,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1727,"record_offset":15324,"parent_record_offset":15296,"ancestor_record_offset":15112,"inlinee":7399,"source_line":97,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1727,"record_offset":15660,"parent_record_offset":15256,"ancestor_record_offset":15112,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1727,"record_offset":15172,"parent_record_offset":15144,"ancestor_record_offset":15112,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1727,"record_offset":15352,"parent_record_offset":15324,"ancestor_record_offset":15112,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::initialize
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1727,"record_offset":15384,"parent_record_offset":15352,"ancestor_record_offset":15112,"inlinee":7400,"source_line":61,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1727,"record_offset":15256,"parent_record_offset":15228,"ancestor_record_offset":15112,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1727,"record_offset":15228,"parent_record_offset":15200,"ancestor_record_offset":15112,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::compile_or_load_texture_assets
// 记录形式: compiled_procedure; instances=1
// 源码行: 71
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1727,"record_offset":10208,"rva":24867584,"va":5393576704}
// 编译实例 RVAs: 0x17b7300
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f8f70
//     owner=unresolved_namespace; count=1
//     target_aliases=18 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18605912","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393576704,"raw_hex":"e8a818e4fe","size":5,"soft_fail":false,"target_va":5374971760,"va":5393577667,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=10
//     target_aliases=1 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18495477","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393576704,"raw_hex":"e80bc8e5fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5393576976,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d61b00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10836040","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393576704,"raw_hex":"e8b8a75aff","size":5,"soft_fail":false,"target_va":5382740736,"va":5393576771,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10749331","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393576704,"raw_hex":"e86dfa5bff","size":5,"soft_fail":false,"target_va":5382827728,"va":5393577054,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420d7100
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9567534","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393576704,"raw_hex":"e82efd9100","size":5,"soft_fail":false,"target_va":5403144448,"va":5393576909,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420d7300
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9568149","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393576704,"raw_hex":"e895ff9100","size":5,"soft_fail":false,"target_va":5403144960,"va":5393576806,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420d7f60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9571227","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393576704,"raw_hex":"e89b0b9200","size":5,"soft_fail":false,"target_va":5403148128,"va":5393576896,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] asset_pipe::texture::compile_texture_assets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t573","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393576704,"raw_hex":"e83d020000","size":5,"soft_fail":false,"target_va":5393577872,"va":5393577294,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read_to_string::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-10871326","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393576704,"raw_hex":"e8e21d5aff","size":5,"soft_fail":false,"target_va":5382705472,"va":5393576793,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::stat
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-10800482","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393576704,"raw_hex":"e89e325bff","size":5,"soft_fail":false,"target_va":5382776592,"va":5393577069,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5393576704,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5393577146,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1727,"record_offset":10376,"parent_record_offset":10208,"ancestor_record_offset":10208,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1727,"record_offset":10672,"parent_record_offset":10208,"ancestor_record_offset":10208,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1727,"record_offset":12516,"parent_record_offset":10208,"ancestor_record_offset":10208,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1727,"record_offset":10488,"parent_record_offset":10208,"ancestor_record_offset":10208,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1727,"record_offset":10520,"parent_record_offset":10208,"ancestor_record_offset":10208,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] ron::de::from_str
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":1727,"record_offset":10548,"parent_record_offset":10208,"ancestor_record_offset":10208,"inlinee":13006,"source_line":108,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1727,"record_offset":10460,"parent_record_offset":10208,"ancestor_record_offset":10208,"inlinee":9324,"source_line":310,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::exists
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1727,"record_offset":11268,"parent_record_offset":10208,"ancestor_record_offset":10208,"inlinee":13003,"source_line":3025,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1727,"record_offset":11128,"parent_record_offset":10208,"ancestor_record_offset":10208,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1727,"record_offset":10848,"parent_record_offset":10824,"ancestor_record_offset":10208,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1727,"record_offset":10824,"parent_record_offset":10768,"ancestor_record_offset":10208,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1727,"record_offset":11708,"parent_record_offset":11680,"ancestor_record_offset":10208,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1727,"record_offset":10796,"parent_record_offset":10768,"ancestor_record_offset":10208,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1727,"record_offset":10768,"parent_record_offset":10744,"ancestor_record_offset":10208,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1727,"record_offset":10744,"parent_record_offset":10720,"ancestor_record_offset":10208,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1727,"record_offset":12628,"parent_record_offset":12604,"ancestor_record_offset":10208,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture::compile_or_load_texture_assets::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":1727,"record_offset":12552,"parent_record_offset":12516,"ancestor_record_offset":10208,"inlinee":77711,"source_line":85,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1727,"record_offset":11740,"parent_record_offset":11708,"ancestor_record_offset":10208,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1727,"record_offset":11764,"parent_record_offset":11740,"ancestor_record_offset":10208,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1727,"record_offset":13072,"parent_record_offset":13044,"ancestor_record_offset":10208,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1727,"record_offset":10696,"parent_record_offset":10672,"ancestor_record_offset":10208,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1727,"record_offset":11508,"parent_record_offset":11480,"ancestor_record_offset":10208,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1727,"record_offset":11556,"parent_record_offset":11532,"ancestor_record_offset":10208,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1727,"record_offset":11532,"parent_record_offset":11508,"ancestor_record_offset":10208,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1727,"record_offset":11332,"parent_record_offset":11304,"ancestor_record_offset":10208,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] ron::options::impl$0::default
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/options.rs
//     evidence={"module":1727,"record_offset":10572,"parent_record_offset":10548,"ancestor_record_offset":10208,"inlinee":12914,"source_line":51,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_os_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1727,"record_offset":10424,"parent_record_offset":10400,"ancestor_record_offset":10208,"inlinee":5171,"source_line":926,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1727,"record_offset":11176,"parent_record_offset":11152,"ancestor_record_offset":10208,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1727,"record_offset":11224,"parent_record_offset":11200,"ancestor_record_offset":10208,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1727,"record_offset":11200,"parent_record_offset":11176,"ancestor_record_offset":10208,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::metadata
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1727,"record_offset":11304,"parent_record_offset":11268,"ancestor_record_offset":10208,"inlinee":12458,"source_line":2381,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":1727,"record_offset":11480,"parent_record_offset":11452,"ancestor_record_offset":10208,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":1727,"record_offset":11452,"parent_record_offset":11424,"ancestor_record_offset":10208,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1727,"record_offset":11152,"parent_record_offset":11128,"ancestor_record_offset":10208,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$37::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1727,"record_offset":10400,"parent_record_offset":10376,"ancestor_record_offset":10208,"inlinee":5169,"source_line":1825,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::compile_or_load_texture_assets::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 79
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A651
// 首条位置: {"module":3165,"inlinee":108113}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":3165,"record_offset":2804,"parent_record_offset":2784,"ancestor_record_offset":2784,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/03165.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":3165,"record_offset":2852,"parent_record_offset":2828,"ancestor_record_offset":2784,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/03165.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3165,"record_offset":2828,"parent_record_offset":2804,"ancestor_record_offset":2784,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/03165.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::compile_or_load_texture_assets::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A650
// 首条位置: {"module":3165,"inlinee":108112}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":3165,"record_offset":2136,"parent_record_offset":2116,"ancestor_record_offset":2116,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/03165.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":3165,"record_offset":2184,"parent_record_offset":2160,"ancestor_record_offset":2116,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/03165.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3165,"record_offset":2160,"parent_record_offset":2136,"ancestor_record_offset":2116,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/03165.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::compile_or_load_texture_assets::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 85
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12F8F
// 首条位置: {"module":1727,"inlinee":77711}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::compile_texture_assets
// 记录形式: compiled_procedure; instances=1
// 源码行: 94
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1727,"record_offset":13728,"rva":24868752,"va":5393577872}
// 编译实例 RVAs: 0x17b7790
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..string..String$u20$as$u20$core..clone..Clone$GT$::clone
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-10732449","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393577872,"raw_hex":"e85f3c5cff","size":5,"soft_fail":false,"target_va":5382847888,"va":5393580332,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-10934353","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393577872,"raw_hex":"e8af2759ff","size":5,"soft_fail":false,"target_va":5382644528,"va":5393578876,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=2
//     target_aliases=92 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18681969","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393577872,"raw_hex":"e88fefe2fe","size":5,"soft_fail":false,"target_va":5374900736,"va":5393582700,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f8f70
//     owner=unresolved_namespace; count=1
//     target_aliases=18 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18611343","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393577872,"raw_hex":"e87103e4fe","size":5,"soft_fail":false,"target_va":5374971760,"va":5393583098,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=26
//     target_aliases=1 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18496585","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393577872,"raw_hex":"e8b7c3e5fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5393578084,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=2
//     target_aliases=69 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18062662","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393577872,"raw_hex":"e8ba62ecfe","size":5,"soft_fail":false,"target_va":5375516032,"va":5393578689,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140889e60
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15917915","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393577872,"raw_hex":"e8a51c0dff","size":5,"soft_fail":false,"target_va":5377662560,"va":5393580470,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b73a50
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12863945","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393577872,"raw_hex":"e837b63bff","size":5,"soft_fail":false,"target_va":5380717136,"va":5393581076,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d61b00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10837215","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393577872,"raw_hex":"e821a35aff","size":5,"soft_fail":false,"target_va":5382740736,"va":5393577946,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=5
//     target_aliases=5 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10750231","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393577872,"raw_hex":"e8e9f65bff","size":5,"soft_fail":false,"target_va":5382827728,"va":5393577954,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420d7b80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9568633","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393577872,"raw_hex":"e879019200","size":5,"soft_fail":false,"target_va":5403147136,"va":5393578498,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420d8310
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9567429","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393577872,"raw_hex":"e8c5fc9100","size":5,"soft_fail":false,"target_va":5403149072,"va":5393581638,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420d8640
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9568991","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393577872,"raw_hex":"e8df029200","size":5,"soft_fail":false,"target_va":5403149888,"va":5393580892,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420dbb30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9584233","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393577872,"raw_hex":"e8693e9200","size":5,"soft_fail":false,"target_va":5403163440,"va":5393579202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420dbef0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9582833","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393577872,"raw_hex":"e8f1389200","size":5,"soft_fail":false,"target_va":5403164400,"va":5393581562,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420dc1a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9585096","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393577872,"raw_hex":"e8c8419200","size":5,"soft_fail":false,"target_va":5403165088,"va":5393579987,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420dd5e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9590296","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393577872,"raw_hex":"e818569200","size":5,"soft_fail":false,"target_va":5403170272,"va":5393579971,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::construct::hd99151b32e25dfd8 (.llvm.2514920207462972046)
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t18209856","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393577872,"raw_hex":"e840dc1501","size":5,"soft_fail":false,"target_va":5411788192,"va":5393578331,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..convert..From$LT$E$GT$$u20$for$u20$anyhow..Error$GT$::from
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t18208143","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393577872,"raw_hex":"e88fd51501","size":5,"soft_fail":false,"target_va":5411789168,"va":5393581020,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-10923977","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393577872,"raw_hex":"e8375059ff","size":5,"soft_fail":false,"target_va":5382655024,"va":5393578996,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] asset::texture::scan::scan_asset_tree
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9572208","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393577872,"raw_hex":"e8700f9200","size":5,"soft_fail":false,"target_va":5403151264,"va":5393579051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t17959609","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393577872,"raw_hex":"e8b90a1201","size":5,"soft_fail":false,"target_va":5411541776,"va":5393582162,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$alloc..vec..into_iter..IntoIter$LT$$LP$relative_path..RelativePathBuf$C$core..result..Result$LT$asset_pipe..texture..TextureAssetMeta$C$anyhow..Error$GT$$RP$$GT$$GT$
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t9550812","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393577872,"raw_hex":"e8dcbb9100","size":5,"soft_fail":false,"target_va":5403131424,"va":5393580607,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-15423","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393577872,"raw_hex":"e8c1c3ffff","size":5,"soft_fail":false,"target_va":5393568224,"va":5393583642,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<ron::error::Error> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-15608","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393577872,"raw_hex":"e808c3ffff","size":5,"soft_fail":false,"target_va":5393568256,"va":5393583859,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t17960593","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393577872,"raw_hex":"e8910e1201","size":5,"soft_fail":false,"target_va":5411542880,"va":5393582282,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::sort::shared::smallsort::insertion_sort_shift_left
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t9593650","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393577872,"raw_hex":"e832639200","size":5,"soft_fail":false,"target_va":5403175760,"va":5393582105,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::sort::stable::driftsort_main
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t9597311","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393577872,"raw_hex":"e87f719200","size":5,"soft_fail":false,"target_va":5403179632,"va":5393582316,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] hashbrown::map::HashMap$LT$K$C$V$C$S$C$A$GT$::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9592591","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393577872,"raw_hex":"e80f5f9200","size":5,"soft_fail":false,"target_va":5403173040,"va":5393580444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9590258","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393577872,"raw_hex":"e8f2559200","size":5,"soft_fail":false,"target_va":5403174032,"va":5393583769,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::backtrace::Backtrace::capture
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-10883578","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393577872,"raw_hex":"e806ee59ff","size":5,"soft_fail":false,"target_va":5382694656,"va":5393578229,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirBuilder::_create
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-10871109","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393577872,"raw_hex":"e8bb1e5aff","size":5,"soft_fail":false,"target_va":5382709584,"va":5393580688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read_to_string::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-10872710","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393577872,"raw_hex":"e87a185aff","size":5,"soft_fail":false,"target_va":5382705472,"va":5393578177,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::write::inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-10874751","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393577872,"raw_hex":"e881105aff","size":5,"soft_fail":false,"target_va":5382706224,"va":5393580970,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-10920261","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393577872,"raw_hex":"e8bb5e59ff","size":5,"soft_fail":false,"target_va":5382657856,"va":5393578112,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::io::stdio::_print
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-10859779","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393577872,"raw_hex":"e8fd4a5aff","size":5,"soft_fail":false,"target_va":5382719888,"va":5393579662,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::parent
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-10839459","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393577872,"raw_hex":"e85d9a5aff","size":5,"soft_fail":false,"target_va":5382741200,"va":5393580654,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::stat
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-10801385","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393577872,"raw_hex":"e8172f5bff","size":5,"soft_fail":false,"target_va":5382776592,"va":5393577972,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-10824557","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393577872,"raw_hex":"e893d45aff","size":5,"soft_fail":false,"target_va":5382753792,"va":5393578344,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5393577872,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5393578055,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$0::sort
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1727,"record_offset":18792,"parent_record_offset":13728,"ancestor_record_offset":13728,"inlinee":6034,"source_line":238,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1727,"record_offset":21072,"parent_record_offset":13728,"ancestor_record_offset":13728,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1727,"record_offset":21160,"parent_record_offset":13728,"ancestor_record_offset":13728,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":1727,"record_offset":22500,"parent_record_offset":13728,"ancestor_record_offset":13728,"inlinee":8212,"source_line":46,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$0::with_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":1727,"record_offset":16092,"parent_record_offset":13728,"ancestor_record_offset":13728,"inlinee":12467,"source_line":58,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] asset_pipe::texture::impl$1::default
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":1727,"record_offset":15112,"parent_record_offset":13728,"ancestor_record_offset":13728,"inlinee":77713,"source_line":38,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1727,"record_offset":19056,"parent_record_offset":13728,"ancestor_record_offset":13728,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1727,"record_offset":18064,"parent_record_offset":13728,"ancestor_record_offset":13728,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1727,"record_offset":19640,"parent_record_offset":13728,"ancestor_record_offset":13728,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::cloned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1727,"record_offset":18680,"parent_record_offset":13728,"ancestor_record_offset":13728,"inlinee":8467,"source_line":3410,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1727,"record_offset":18736,"parent_record_offset":13728,"ancestor_record_offset":13728,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1727,"record_offset":20988,"parent_record_offset":13728,"ancestor_record_offset":13728,"inlinee":5102,"source_line":745,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1727,"record_offset":22380,"parent_record_offset":13728,"ancestor_record_offset":13728,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1727,"record_offset":19292,"parent_record_offset":13728,"ancestor_record_offset":13728,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1727,"record_offset":16216,"parent_record_offset":13728,"ancestor_record_offset":13728,"inlinee":4667,"source_line":1351,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1727,"record_offset":22668,"parent_record_offset":13728,"ancestor_record_offset":13728,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1727,"record_offset":16900,"parent_record_offset":13728,"ancestor_record_offset":13728,"inlinee":5122,"source_line":1460,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1727,"record_offset":18288,"parent_record_offset":13728,"ancestor_record_offset":13728,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1727,"record_offset":19264,"parent_record_offset":13728,"ancestor_record_offset":13728,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] easy_parallel::Parallel::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"easy-parallel","version":"3.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/easy-parallel-3.2.0/src/lib.rs
//     evidence={"module":1727,"record_offset":21044,"parent_record_offset":13728,"ancestor_record_offset":13728,"inlinee":19622,"source_line":85,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1727,"record_offset":19088,"parent_record_offset":13728,"ancestor_record_offset":13728,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1727,"record_offset":18972,"parent_record_offset":13728,"ancestor_record_offset":13728,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] relative_path::impl$82::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"relative-path","version":"1.9.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/relative-path-1.9.0/src/lib.rs
//     evidence={"module":1727,"record_offset":21300,"parent_record_offset":13728,"ancestor_record_offset":13728,"inlinee":28751,"source_line":631,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] ron::ser::to_string
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/ser/mod.rs
//     evidence={"module":1727,"record_offset":22612,"parent_record_offset":13728,"ancestor_record_offset":13728,"inlinee":77717,"source_line":44,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1727,"record_offset":21360,"parent_record_offset":13728,"ancestor_record_offset":13728,"inlinee":4947,"source_line":1189,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::keys
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1727,"record_offset":18320,"parent_record_offset":13728,"ancestor_record_offset":13728,"inlinee":12667,"source_line":378,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$46::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1727,"record_offset":20400,"parent_record_offset":13728,"ancestor_record_offset":13728,"inlinee":10297,"source_line":2382,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] std::fs::create_dir_all
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1727,"record_offset":22416,"parent_record_offset":13728,"ancestor_record_offset":13728,"inlinee":8207,"source_line":2766,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1727,"record_offset":15868,"parent_record_offset":13728,"ancestor_record_offset":13728,"inlinee":9324,"source_line":310,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] std::fs::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1727,"record_offset":22712,"parent_record_offset":13728,"ancestor_record_offset":13728,"inlinee":9318,"source_line":347,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::exists
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1727,"record_offset":14180,"parent_record_offset":13728,"ancestor_record_offset":13728,"inlinee":13003,"source_line":3025,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$37::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1727,"record_offset":13984,"parent_record_offset":13728,"ancestor_record_offset":13728,"inlinee":5169,"source_line":1825,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1727,"record_offset":14040,"parent_record_offset":13728,"ancestor_record_offset":13728,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1727,"record_offset":14728,"parent_record_offset":14704,"ancestor_record_offset":13728,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1727,"record_offset":14704,"parent_record_offset":14616,"ancestor_record_offset":13728,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1727,"record_offset":14616,"parent_record_offset":14588,"ancestor_record_offset":13728,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1727,"record_offset":16528,"parent_record_offset":16496,"ancestor_record_offset":13728,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1727,"record_offset":16496,"parent_record_offset":16468,"ancestor_record_offset":13728,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1727,"record_offset":16468,"parent_record_offset":16440,"ancestor_record_offset":13728,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1727,"record_offset":18828,"parent_record_offset":18792,"ancestor_record_offset":13728,"inlinee":6036,"source_line":907,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1727,"record_offset":18760,"parent_record_offset":18736,"ancestor_record_offset":13728,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1727,"record_offset":23592,"parent_record_offset":23568,"ancestor_record_offset":13728,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":1727,"record_offset":16176,"parent_record_offset":16136,"ancestor_record_offset":13728,"inlinee":7901,"source_line":173,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":1727,"record_offset":16136,"parent_record_offset":16092,"ancestor_record_offset":13728,"inlinee":7918,"source_line":23,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture::compile_texture_assets::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":1727,"record_offset":16252,"parent_record_offset":16216,"ancestor_record_offset":13728,"inlinee":77715,"source_line":104,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture::compile_texture_assets::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":1727,"record_offset":16936,"parent_record_offset":16900,"ancestor_record_offset":13728,"inlinee":77716,"source_line":105,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":1727,"record_offset":17268,"parent_record_offset":16936,"ancestor_record_offset":13728,"inlinee":77713,"source_line":38,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1727,"record_offset":14648,"parent_record_offset":14616,"ancestor_record_offset":13728,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1727,"record_offset":15692,"parent_record_offset":15660,"ancestor_record_offset":13728,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1727,"record_offset":15752,"parent_record_offset":15720,"ancestor_record_offset":13728,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1727,"record_offset":17060,"parent_record_offset":16936,"ancestor_record_offset":13728,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1727,"record_offset":15892,"parent_record_offset":15868,"ancestor_record_offset":13728,"inlinee":5126,"source_line":703,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1727,"record_offset":18532,"parent_record_offset":18504,"ancestor_record_offset":13728,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1727,"record_offset":18592,"parent_record_offset":18560,"ancestor_record_offset":13728,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1727,"record_offset":18904,"parent_record_offset":18864,"ancestor_record_offset":13728,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::cloned::Cloned::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\cloned.rs
//     evidence={"module":1727,"record_offset":18704,"parent_record_offset":18680,"ancestor_record_offset":13728,"inlinee":8469,"source_line":23,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1727,"record_offset":21012,"parent_record_offset":20988,"ancestor_record_offset":13728,"inlinee":5104,"source_line":68,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":1727,"record_offset":25092,"parent_record_offset":25064,"ancestor_record_offset":13728,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\manually_drop.rs
//     evidence={"module":1727,"record_offset":21100,"parent_record_offset":21072,"ancestor_record_offset":13728,"inlinee":5075,"source_line":180,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1727,"record_offset":15548,"parent_record_offset":15524,"ancestor_record_offset":13728,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1727,"record_offset":14672,"parent_record_offset":14648,"ancestor_record_offset":13728,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1727,"record_offset":20908,"parent_record_offset":20820,"ancestor_record_offset":13728,"inlinee":4812,"source_line":486,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1727,"record_offset":20848,"parent_record_offset":20820,"ancestor_record_offset":13728,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1727,"record_offset":20876,"parent_record_offset":20848,"ancestor_record_offset":13728,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1727,"record_offset":24036,"parent_record_offset":24008,"ancestor_record_offset":13728,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1727,"record_offset":15720,"parent_record_offset":15660,"ancestor_record_offset":13728,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1727,"record_offset":15296,"parent_record_offset":15256,"ancestor_record_offset":13728,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1727,"record_offset":15412,"parent_record_offset":15384,"ancestor_record_offset":13728,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1727,"record_offset":17148,"parent_record_offset":17116,"ancestor_record_offset":13728,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1727,"record_offset":17176,"parent_record_offset":17116,"ancestor_record_offset":13728,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1727,"record_offset":18476,"parent_record_offset":18444,"ancestor_record_offset":13728,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=97
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1727,"record_offset":14276,"parent_record_offset":14180,"ancestor_record_offset":13728,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1727,"record_offset":21128,"parent_record_offset":21072,"ancestor_record_offset":13728,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1727,"record_offset":15500,"parent_record_offset":15384,"ancestor_record_offset":13728,"inlinee":7403,"source_line":1581,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1727,"record_offset":20940,"parent_record_offset":20788,"ancestor_record_offset":13728,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1727,"record_offset":14416,"parent_record_offset":14388,"ancestor_record_offset":13728,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1727,"record_offset":14464,"parent_record_offset":14440,"ancestor_record_offset":13728,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1727,"record_offset":14440,"parent_record_offset":14416,"ancestor_record_offset":13728,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1727,"record_offset":21240,"parent_record_offset":21160,"ancestor_record_offset":13728,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1727,"record_offset":21208,"parent_record_offset":21160,"ancestor_record_offset":13728,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1727,"record_offset":15576,"parent_record_offset":15548,"ancestor_record_offset":13728,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1727,"record_offset":15524,"parent_record_offset":15500,"ancestor_record_offset":13728,"inlinee":7404,"source_line":1212,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1727,"record_offset":15604,"parent_record_offset":15548,"ancestor_record_offset":13728,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1727,"record_offset":14244,"parent_record_offset":14216,"ancestor_record_offset":13728,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1727,"record_offset":16348,"parent_record_offset":16252,"ancestor_record_offset":13728,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1727,"record_offset":16636,"parent_record_offset":16252,"ancestor_record_offset":13728,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":1727,"record_offset":18864,"parent_record_offset":18828,"ancestor_record_offset":13728,"inlinee":6038,"source_line":29,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1727,"record_offset":17000,"parent_record_offset":16976,"ancestor_record_offset":13728,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1727,"record_offset":17024,"parent_record_offset":17000,"ancestor_record_offset":13728,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1727,"record_offset":18504,"parent_record_offset":18444,"ancestor_record_offset":13728,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1727,"record_offset":18560,"parent_record_offset":18504,"ancestor_record_offset":13728,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1727,"record_offset":18368,"parent_record_offset":18344,"ancestor_record_offset":13728,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$42::into_iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1727,"record_offset":20424,"parent_record_offset":20400,"ancestor_record_offset":13728,"inlinee":4805,"source_line":3151,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1727,"record_offset":18444,"parent_record_offset":18416,"ancestor_record_offset":13728,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::into_allocation
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1727,"record_offset":20788,"parent_record_offset":20764,"ancestor_record_offset":13728,"inlinee":4809,"source_line":1358,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::into_iter_from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1727,"record_offset":20764,"parent_record_offset":20448,"ancestor_record_offset":13728,"inlinee":4808,"source_line":1344,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1727,"record_offset":18392,"parent_record_offset":18368,"ancestor_record_offset":13728,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1727,"record_offset":20536,"parent_record_offset":20508,"ancestor_record_offset":13728,"inlinee":4952,"source_line":2488,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1727,"record_offset":24744,"parent_record_offset":24708,"ancestor_record_offset":13728,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1727,"record_offset":18416,"parent_record_offset":18392,"ancestor_record_offset":13728,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1727,"record_offset":20820,"parent_record_offset":20788,"ancestor_record_offset":13728,"inlinee":4811,"source_line":162,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1727,"record_offset":23524,"parent_record_offset":23500,"ancestor_record_offset":13728,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$19::into_iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1727,"record_offset":20448,"parent_record_offset":20424,"ancestor_record_offset":13728,"inlinee":4807,"source_line":3343,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1727,"record_offset":17204,"parent_record_offset":17116,"ancestor_record_offset":13728,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1727,"record_offset":17092,"parent_record_offset":16936,"ancestor_record_offset":13728,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1727,"record_offset":17116,"parent_record_offset":17092,"ancestor_record_offset":13728,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1727,"record_offset":17228,"parent_record_offset":17204,"ancestor_record_offset":13728,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1727,"record_offset":16976,"parent_record_offset":16936,"ancestor_record_offset":13728,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::from_str
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":1727,"record_offset":16288,"parent_record_offset":16252,"ancestor_record_offset":13728,"inlinee":13006,"source_line":108,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] ron::options::impl$0::default
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/options.rs
//     evidence={"module":1727,"record_offset":16312,"parent_record_offset":16288,"ancestor_record_offset":13728,"inlinee":12914,"source_line":51,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1727,"record_offset":18344,"parent_record_offset":18320,"ancestor_record_offset":13728,"inlinee":4949,"source_line":535,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1727,"record_offset":15144,"parent_record_offset":15112,"ancestor_record_offset":13728,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_os_string
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1727,"record_offset":14008,"parent_record_offset":13984,"ancestor_record_offset":13728,"inlinee":5171,"source_line":926,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1727,"record_offset":14088,"parent_record_offset":14064,"ancestor_record_offset":13728,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1727,"record_offset":14136,"parent_record_offset":14112,"ancestor_record_offset":13728,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1727,"record_offset":14112,"parent_record_offset":14088,"ancestor_record_offset":13728,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1727,"record_offset":22468,"parent_record_offset":22416,"ancestor_record_offset":13728,"inlinee":8210,"source_line":3032,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1727,"record_offset":22440,"parent_record_offset":22416,"ancestor_record_offset":13728,"inlinee":8209,"source_line":3008,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::metadata
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1727,"record_offset":14216,"parent_record_offset":14180,"ancestor_record_offset":13728,"inlinee":12458,"source_line":2381,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1727,"record_offset":15200,"parent_record_offset":15172,"ancestor_record_offset":13728,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__init
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1727,"record_offset":15460,"parent_record_offset":15436,"ancestor_record_offset":13728,"inlinee":7402,"source_line":83,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1727,"record_offset":15324,"parent_record_offset":15296,"ancestor_record_offset":13728,"inlinee":7399,"source_line":97,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1727,"record_offset":15660,"parent_record_offset":15256,"ancestor_record_offset":13728,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1727,"record_offset":15172,"parent_record_offset":15144,"ancestor_record_offset":13728,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":1727,"record_offset":14388,"parent_record_offset":14360,"ancestor_record_offset":13728,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":1727,"record_offset":14360,"parent_record_offset":14332,"ancestor_record_offset":13728,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1727,"record_offset":14064,"parent_record_offset":14040,"ancestor_record_offset":13728,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$37::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1727,"record_offset":18088,"parent_record_offset":18064,"ancestor_record_offset":13728,"inlinee":5169,"source_line":1825,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1727,"record_offset":15940,"parent_record_offset":15916,"ancestor_record_offset":13728,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1727,"record_offset":15916,"parent_record_offset":15892,"ancestor_record_offset":13728,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1727,"record_offset":15352,"parent_record_offset":15324,"ancestor_record_offset":13728,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::initialize
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1727,"record_offset":15384,"parent_record_offset":15352,"ancestor_record_offset":13728,"inlinee":7400,"source_line":61,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1727,"record_offset":15256,"parent_record_offset":15228,"ancestor_record_offset":13728,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1727,"record_offset":15228,"parent_record_offset":15200,"ancestor_record_offset":13728,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::compile_texture_assets::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 104
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12F93
// 首条位置: {"module":1727,"inlinee":77715}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1727,"record_offset":16384,"parent_record_offset":16252,"ancestor_record_offset":16252,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1727,"record_offset":16348,"parent_record_offset":16252,"ancestor_record_offset":16252,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1727,"record_offset":16636,"parent_record_offset":16252,"ancestor_record_offset":16252,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] ron::de::from_str
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":1727,"record_offset":16288,"parent_record_offset":16252,"ancestor_record_offset":16252,"inlinee":13006,"source_line":108,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1727,"record_offset":16584,"parent_record_offset":16560,"ancestor_record_offset":16252,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1727,"record_offset":16560,"parent_record_offset":16496,"ancestor_record_offset":16252,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1727,"record_offset":16528,"parent_record_offset":16496,"ancestor_record_offset":16252,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1727,"record_offset":16496,"parent_record_offset":16468,"ancestor_record_offset":16252,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1727,"record_offset":16468,"parent_record_offset":16440,"ancestor_record_offset":16252,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1727,"record_offset":16412,"parent_record_offset":16384,"ancestor_record_offset":16252,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] ron::options::impl$0::default
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/options.rs
//     evidence={"module":1727,"record_offset":16312,"parent_record_offset":16288,"ancestor_record_offset":16252,"inlinee":12914,"source_line":51,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::compile_texture_assets::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 105
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12F94
// 首条位置: {"module":1727,"inlinee":77716}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset_pipe::texture::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":1727,"record_offset":17268,"parent_record_offset":16936,"ancestor_record_offset":16936,"inlinee":77713,"source_line":38,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1727,"record_offset":17060,"parent_record_offset":16936,"ancestor_record_offset":16936,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1727,"record_offset":18000,"parent_record_offset":16936,"ancestor_record_offset":16936,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1727,"record_offset":17092,"parent_record_offset":16936,"ancestor_record_offset":16936,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1727,"record_offset":16976,"parent_record_offset":16936,"ancestor_record_offset":16936,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1727,"record_offset":17824,"parent_record_offset":17792,"ancestor_record_offset":16936,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1727,"record_offset":17884,"parent_record_offset":17852,"ancestor_record_offset":16936,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1727,"record_offset":17680,"parent_record_offset":17656,"ancestor_record_offset":16936,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1727,"record_offset":17852,"parent_record_offset":17792,"ancestor_record_offset":16936,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1727,"record_offset":17428,"parent_record_offset":17392,"ancestor_record_offset":16936,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1727,"record_offset":17544,"parent_record_offset":17516,"ancestor_record_offset":16936,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1727,"record_offset":17148,"parent_record_offset":17116,"ancestor_record_offset":16936,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1727,"record_offset":17176,"parent_record_offset":17116,"ancestor_record_offset":16936,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1727,"record_offset":17632,"parent_record_offset":17516,"ancestor_record_offset":16936,"inlinee":7403,"source_line":1581,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1727,"record_offset":17708,"parent_record_offset":17680,"ancestor_record_offset":16936,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1727,"record_offset":17656,"parent_record_offset":17632,"ancestor_record_offset":16936,"inlinee":7404,"source_line":1212,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1727,"record_offset":17736,"parent_record_offset":17680,"ancestor_record_offset":16936,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1727,"record_offset":17000,"parent_record_offset":16976,"ancestor_record_offset":16936,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1727,"record_offset":17024,"parent_record_offset":17000,"ancestor_record_offset":16936,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1727,"record_offset":17204,"parent_record_offset":17116,"ancestor_record_offset":16936,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1727,"record_offset":17116,"parent_record_offset":17092,"ancestor_record_offset":16936,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1727,"record_offset":17228,"parent_record_offset":17204,"ancestor_record_offset":16936,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1727,"record_offset":17296,"parent_record_offset":17268,"ancestor_record_offset":16936,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1727,"record_offset":17344,"parent_record_offset":17320,"ancestor_record_offset":16936,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1727,"record_offset":17592,"parent_record_offset":17568,"ancestor_record_offset":16936,"inlinee":7402,"source_line":83,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1727,"record_offset":17456,"parent_record_offset":17428,"ancestor_record_offset":16936,"inlinee":7399,"source_line":97,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1727,"record_offset":17792,"parent_record_offset":17392,"ancestor_record_offset":16936,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1727,"record_offset":17320,"parent_record_offset":17296,"ancestor_record_offset":16936,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1727,"record_offset":17484,"parent_record_offset":17456,"ancestor_record_offset":16936,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::initialize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1727,"record_offset":17516,"parent_record_offset":17484,"ancestor_record_offset":16936,"inlinee":7400,"source_line":61,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1727,"record_offset":17392,"parent_record_offset":17368,"ancestor_record_offset":16936,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1727,"record_offset":17368,"parent_record_offset":17344,"ancestor_record_offset":16936,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::compile_texture_assets::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 141
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A6D7
// 首条位置: {"module":3187,"inlinee":108247}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3187,"record_offset":940,"parent_record_offset":532,"ancestor_record_offset":532,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":3187,"record_offset":560,"parent_record_offset":532,"ancestor_record_offset":532,"inlinee":8292,"source_line":1244,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3187,"record_offset":876,"parent_record_offset":852,"ancestor_record_offset":532,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3187,"record_offset":852,"parent_record_offset":796,"ancestor_record_offset":532,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3187,"record_offset":824,"parent_record_offset":796,"ancestor_record_offset":532,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3187,"record_offset":796,"parent_record_offset":772,"ancestor_record_offset":532,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3187,"record_offset":772,"parent_record_offset":748,"ancestor_record_offset":532,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3187,"record_offset":676,"parent_record_offset":584,"ancestor_record_offset":532,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":3187,"record_offset":584,"parent_record_offset":560,"ancestor_record_offset":532,"inlinee":5625,"source_line":1954,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":3187,"record_offset":616,"parent_record_offset":584,"ancestor_record_offset":532,"inlinee":5626,"source_line":1993,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":3187,"record_offset":644,"parent_record_offset":616,"ancestor_record_offset":532,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::compile_texture_assets::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 145
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A67B
// 首条位置: {"module":3169,"inlinee":108155}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3169,"record_offset":9068,"parent_record_offset":8992,"ancestor_record_offset":8992,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03169.jsonl.gz"}
//   [pdb_inline_direct] relative_path::impl$82::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"relative-path","version":"1.9.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/relative-path-1.9.0/src/lib.rs
//     evidence={"module":3169,"record_offset":9040,"parent_record_offset":8992,"ancestor_record_offset":8992,"inlinee":28751,"source_line":631,"archive":"_pdb/B/modules/03169.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3169,"record_offset":9268,"parent_record_offset":9244,"ancestor_record_offset":8992,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/03169.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3169,"record_offset":9244,"parent_record_offset":9188,"ancestor_record_offset":8992,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/03169.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3169,"record_offset":9216,"parent_record_offset":9188,"ancestor_record_offset":8992,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/03169.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3169,"record_offset":9188,"parent_record_offset":9164,"ancestor_record_offset":8992,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/03169.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3169,"record_offset":9164,"parent_record_offset":9140,"ancestor_record_offset":8992,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/03169.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3169,"record_offset":9092,"parent_record_offset":9068,"ancestor_record_offset":8992,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03169.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::compile_texture_asset::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 231
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A656
// 首条位置: {"module":3165,"inlinee":108118}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":3165,"record_offset":5484,"parent_record_offset":5464,"ancestor_record_offset":5464,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/03165.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":3165,"record_offset":5532,"parent_record_offset":5508,"ancestor_record_offset":5464,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/03165.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3165,"record_offset":5508,"parent_record_offset":5484,"ancestor_record_offset":5464,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/03165.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::resize_image
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 247
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12FBF
// 首条位置: {"module":1727,"inlinee":77759}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::pad_image
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 306
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12FC0
// 首条位置: {"module":1727,"inlinee":77760}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset_pipe::texture::resize_image
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":1727,"record_offset":52164,"parent_record_offset":52068,"ancestor_record_offset":52068,"inlinee":77759,"source_line":247,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1727,"record_offset":52476,"parent_record_offset":52068,"ancestor_record_offset":52068,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1727,"record_offset":52932,"parent_record_offset":52068,"ancestor_record_offset":52068,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] image::image::GenericImage::copy_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/image.rs
//     evidence={"module":1727,"record_offset":52192,"parent_record_offset":52068,"ancestor_record_offset":52068,"inlinee":77762,"source_line":963,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$64::lt
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1727,"record_offset":52292,"parent_record_offset":52260,"ancestor_record_offset":52068,"inlinee":5290,"source_line":1824,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$39::forward_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1727,"record_offset":52324,"parent_record_offset":52260,"ancestor_record_offset":52068,"inlinee":5292,"source_line":203,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1727,"record_offset":52260,"parent_record_offset":52232,"ancestor_record_offset":52068,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1727,"record_offset":52232,"parent_record_offset":52192,"ancestor_record_offset":52068,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::unchecked_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1727,"record_offset":52348,"parent_record_offset":52324,"ancestor_record_offset":52068,"inlinee":5293,"source_line":560,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::process_regular_image
// 记录形式: compiled_procedure; instances=1
// 源码行: 343
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1727,"record_offset":48612,"rva":24886288,"va":5393595408}
// 编译实例 RVAs: 0x17bbc10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18515809","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393595408,"raw_hex":"e89f78e5fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5393597308,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10767770","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393595408,"raw_hex":"e866b25bff","size":5,"soft_fail":false,"target_va":5382827728,"va":5393595493,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420d7480
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9549660","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393595408,"raw_hex":"e85cb79100","size":5,"soft_fail":false,"target_va":5403145344,"va":5393595679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420e2bd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9595336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393595408,"raw_hex":"e8c8699200","size":5,"soft_fail":false,"target_va":5403192272,"va":5393596931,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420e3080
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9596713","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393595408,"raw_hex":"e8296f9200","size":5,"soft_fail":false,"target_va":5403193472,"va":5393596754,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923f70
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18252094","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393595408,"raw_hex":"e83e811601","size":5,"soft_fail":false,"target_va":5411848048,"va":5393595949,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..convert..From$LT$E$GT$$u20$for$u20$anyhow..Error$GT$::from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t18191739","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393595408,"raw_hex":"e87b951501","size":5,"soft_fail":false,"target_va":5411789120,"va":5393597376,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] asset_pipe::texture::process_regular_image::closure$3
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1964","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393595408,"raw_hex":"e8ac070000","size":5,"soft_fail":false,"target_va":5393598768,"va":5393596799,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t17945446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393595408,"raw_hex":"e866d31101","size":5,"soft_fail":false,"target_va":5411541776,"va":5393596325,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t17944957","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393595408,"raw_hex":"e87dd11101","size":5,"soft_fail":false,"target_va":5411542064,"va":5393597102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t17944196","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393595408,"raw_hex":"e884ce1101","size":5,"soft_fail":false,"target_va":5411541904,"va":5393597703,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<image::dynimage::DynamicImage> >
//     owner=rust_standard_library; count=9
//     evidence={"assembly":"\tcallq\t-27865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393595408,"raw_hex":"e82793ffff","size":5,"soft_fail":false,"target_va":5393569104,"va":5393596964,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t17945505","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393595408,"raw_hex":"e8a1d31101","size":5,"soft_fail":false,"target_va":5411543104,"va":5393597594,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] image::dynimage::DynamicImage::into_rgba32f
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"image","version":"0.24.9","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12641102","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393595408,"raw_hex":"e8b21c3fff","size":5,"soft_fail":false,"target_va":5380954720,"va":5393595817,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] image::dynimage::DynamicImage::resize_exact
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"image","version":"0.24.9","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12641732","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393595408,"raw_hex":"e83c1a3fff","size":5,"soft_fail":false,"target_va":5380954832,"va":5393596559,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] image::dynimage::DynamicImage::to_rgba8
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"image","version":"0.24.9","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12641880","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393595408,"raw_hex":"e8a8193fff","size":5,"soft_fail":false,"target_va":5380954320,"va":5393596195,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] image::io::reader::Reader$LT$R$GT$::decode
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"image","version":"0.24.9","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9594095","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393595408,"raw_hex":"e8ef649200","size":5,"soft_fail":false,"target_va":5403189760,"va":5393595660,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] image::io::reader::Reader$LT$std..io..buffered..bufreader..BufReader$LT$std..fs..File$GT$$GT$::open_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"image","version":"0.24.9","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12681836","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393595408,"raw_hex":"e8947d3eff","size":5,"soft_fail":false,"target_va":5380913680,"va":5393595511,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rkyv::util::to_bytes<asset::texture::blob::TextureAssetBlob,256>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rkyv","version":"0.7.39","source":"git+https://github.com/PounceLight/rkyv.git?rev=031d58108ccf8c0e125b06b017f72b4afd67e9a3#031d58108ccf8c0e125b06b017f72b4afd67e9a3"}]
//     evidence={"assembly":"\tcallq\t-28036","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393595408,"raw_hex":"e87c92ffff","size":5,"soft_fail":false,"target_va":5393569216,"va":5393597247,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] asset_pipe::texture::apply_transfer_function
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":1727,"record_offset":49164,"parent_record_offset":48612,"ancestor_record_offset":48612,"inlinee":77751,"source_line":504,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] asset_pipe::texture::resize_image
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":1727,"record_offset":50208,"parent_record_offset":48612,"ancestor_record_offset":48612,"inlinee":77759,"source_line":247,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1727,"record_offset":50236,"parent_record_offset":48612,"ancestor_record_offset":48612,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1727,"record_offset":50264,"parent_record_offset":48612,"ancestor_record_offset":48612,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1727,"record_offset":49956,"parent_record_offset":48612,"ancestor_record_offset":48612,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1727,"record_offset":51008,"parent_record_offset":48612,"ancestor_record_offset":48612,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1727,"record_offset":50292,"parent_record_offset":48612,"ancestor_record_offset":48612,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$8::next_multiple_of
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1727,"record_offset":50096,"parent_record_offset":48612,"ancestor_record_offset":48612,"inlinee":77758,"source_line":3233,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1727,"record_offset":50068,"parent_record_offset":48612,"ancestor_record_offset":48612,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1727,"record_offset":50524,"parent_record_offset":48612,"ancestor_record_offset":48612,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1727,"record_offset":49100,"parent_record_offset":48612,"ancestor_record_offset":48612,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1727,"record_offset":49136,"parent_record_offset":48612,"ancestor_record_offset":48612,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] image::dynimage::open
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/dynimage.rs
//     evidence={"module":1727,"record_offset":48788,"parent_record_offset":48612,"ancestor_record_offset":48612,"inlinee":5991,"source_line":1083,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1727,"record_offset":50700,"parent_record_offset":50676,"ancestor_record_offset":48612,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1727,"record_offset":50676,"parent_record_offset":50620,"ancestor_record_offset":48612,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1727,"record_offset":50648,"parent_record_offset":50620,"ancestor_record_offset":48612,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1727,"record_offset":50620,"parent_record_offset":50596,"ancestor_record_offset":48612,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1727,"record_offset":50596,"parent_record_offset":50572,"ancestor_record_offset":48612,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture::srgb_eotf
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":1727,"record_offset":49752,"parent_record_offset":49164,"ancestor_record_offset":48612,"inlinee":77753,"source_line":524,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$64::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1727,"record_offset":50356,"parent_record_offset":50320,"ancestor_record_offset":48612,"inlinee":5290,"source_line":1824,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1727,"record_offset":48836,"parent_record_offset":48812,"ancestor_record_offset":48612,"inlinee":5126,"source_line":703,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$39::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1727,"record_offset":50388,"parent_record_offset":50320,"ancestor_record_offset":48612,"inlinee":5292,"source_line":203,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1727,"record_offset":50320,"parent_record_offset":50292,"ancestor_record_offset":48612,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1727,"record_offset":49356,"parent_record_offset":49332,"ancestor_record_offset":48612,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1727,"record_offset":49380,"parent_record_offset":49356,"ancestor_record_offset":48612,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1727,"record_offset":51620,"parent_record_offset":51588,"ancestor_record_offset":48612,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1727,"record_offset":50412,"parent_record_offset":50388,"ancestor_record_offset":48612,"inlinee":5293,"source_line":560,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1727,"record_offset":49308,"parent_record_offset":49280,"ancestor_record_offset":48612,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1727,"record_offset":49536,"parent_record_offset":49240,"ancestor_record_offset":48612,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1727,"record_offset":50548,"parent_record_offset":50524,"ancestor_record_offset":48612,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1727,"record_offset":49036,"parent_record_offset":48788,"ancestor_record_offset":48612,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1727,"record_offset":49068,"parent_record_offset":48788,"ancestor_record_offset":48612,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1727,"record_offset":49572,"parent_record_offset":49240,"ancestor_record_offset":48612,"inlinee":5238,"source_line":26,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1727,"record_offset":49628,"parent_record_offset":49600,"ancestor_record_offset":48612,"inlinee":5308,"source_line":444,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1727,"record_offset":49600,"parent_record_offset":49572,"ancestor_record_offset":48612,"inlinee":5312,"source_line":526,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$98::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1727,"record_offset":49712,"parent_record_offset":49680,"ancestor_record_offset":48612,"inlinee":9507,"source_line":2069,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::image_buffer_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":1727,"record_offset":49280,"parent_record_offset":49240,"ancestor_record_offset":48612,"inlinee":7815,"source_line":807,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::inner_pixels_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":1727,"record_offset":49240,"parent_record_offset":49208,"ancestor_record_offset":48612,"inlinee":77752,"source_line":892,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::pixels_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":1727,"record_offset":49208,"parent_record_offset":49164,"ancestor_record_offset":48612,"inlinee":9499,"source_line":898,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$34::image_buffer_len::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":1727,"record_offset":49332,"parent_record_offset":49308,"ancestor_record_offset":48612,"inlinee":7817,"source_line":809,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$34::image_buffer_len::closure$1
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":1727,"record_offset":49444,"parent_record_offset":49420,"ancestor_record_offset":48612,"inlinee":7818,"source_line":810,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$5::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":1727,"record_offset":49680,"parent_record_offset":49164,"ancestor_record_offset":48612,"inlinee":9505,"source_line":98,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] image::dynimage::impl$10::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/dynimage.rs
//     evidence={"module":1727,"record_offset":49980,"parent_record_offset":49956,"ancestor_record_offset":48612,"inlinee":77755,"source_line":929,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] image::dynimage::impl$4::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/dynimage.rs
//     evidence={"module":1727,"record_offset":50036,"parent_record_offset":50012,"ancestor_record_offset":48612,"inlinee":77757,"source_line":893,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] image::io::reader::Reader::open
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/io/reader.rs
//     evidence={"module":1727,"record_offset":48812,"parent_record_offset":48788,"ancestor_record_offset":48612,"inlinee":5206,"source_line":293,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::powf
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1727,"record_offset":49788,"parent_record_offset":49752,"ancestor_record_offset":48612,"inlinee":11915,"source_line":339,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1727,"record_offset":48932,"parent_record_offset":48908,"ancestor_record_offset":48612,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1727,"record_offset":48980,"parent_record_offset":48956,"ancestor_record_offset":48612,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1727,"record_offset":48956,"parent_record_offset":48932,"ancestor_record_offset":48612,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1727,"record_offset":48908,"parent_record_offset":48884,"ancestor_record_offset":48612,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1727,"record_offset":48884,"parent_record_offset":48860,"ancestor_record_offset":48612,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1727,"record_offset":48860,"parent_record_offset":48836,"ancestor_record_offset":48612,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::process_regular_image::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 344
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A652
// 首条位置: {"module":3165,"inlinee":108114}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":3165,"record_offset":3496,"parent_record_offset":3476,"ancestor_record_offset":3476,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/03165.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":3165,"record_offset":3544,"parent_record_offset":3520,"ancestor_record_offset":3476,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/03165.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3165,"record_offset":3520,"parent_record_offset":3496,"ancestor_record_offset":3476,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/03165.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::process_regular_image::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 415
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A6DA
// 首条位置: {"module":3187,"inlinee":108250}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3187,"record_offset":6692,"parent_record_offset":6580,"ancestor_record_offset":6580,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3187,"record_offset":6628,"parent_record_offset":6580,"ancestor_record_offset":6580,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_direct] image::buffer_::impl$41::deref
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":3187,"record_offset":6664,"parent_record_offset":6580,"ancestor_record_offset":6580,"inlinee":9524,"source_line":1110,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3187,"record_offset":7116,"parent_record_offset":7092,"ancestor_record_offset":6580,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3187,"record_offset":7140,"parent_record_offset":7116,"ancestor_record_offset":6580,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3187,"record_offset":7092,"parent_record_offset":6916,"ancestor_record_offset":6580,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3187,"record_offset":6856,"parent_record_offset":6828,"ancestor_record_offset":6580,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3187,"record_offset":6916,"parent_record_offset":6884,"ancestor_record_offset":6580,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3187,"record_offset":6884,"parent_record_offset":6856,"ancestor_record_offset":6580,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3187,"record_offset":6952,"parent_record_offset":6916,"ancestor_record_offset":6580,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3187,"record_offset":6788,"parent_record_offset":6756,"ancestor_record_offset":6580,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3187,"record_offset":6756,"parent_record_offset":6724,"ancestor_record_offset":6580,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3187,"record_offset":6724,"parent_record_offset":6692,"ancestor_record_offset":6580,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3187,"record_offset":6828,"parent_record_offset":6788,"ancestor_record_offset":6580,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":3187,"record_offset":6976,"parent_record_offset":6952,"ancestor_record_offset":6580,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":3187,"record_offset":7000,"parent_record_offset":6976,"ancestor_record_offset":6580,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":3187,"record_offset":7216,"parent_record_offset":7192,"ancestor_record_offset":6580,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":3187,"record_offset":7048,"parent_record_offset":7024,"ancestor_record_offset":6580,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3187,"record_offset":7024,"parent_record_offset":7000,"ancestor_record_offset":6580,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3187,"record_offset":7192,"parent_record_offset":6788,"ancestor_record_offset":6580,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::process_regular_image::closure$3
// 记录形式: compiled_procedure; instances=1
// 源码行: 424
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1727,"record_offset":51760,"rva":24889648,"va":5393598768}
// 编译实例 RVAs: 0x17bc930
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$image..dynimage..DynamicImage$u20$as$u20$image..image..GenericImage$GT$::put_pixel
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-12642671","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393598768,"raw_hex":"e891163fff","size":5,"soft_fail":false,"target_va":5380956432,"va":5393599098,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$image..dynimage..DynamicImage$u20$as$u20$image..image..GenericImageView$GT$::get_pixel
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-12644078","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393598768,"raw_hex":"e812113fff","size":5,"soft_fail":false,"target_va":5380955008,"va":5393599081,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t17942666","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393598768,"raw_hex":"e88ac81101","size":5,"soft_fail":false,"target_va":5411542064,"va":5393599393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_const::panic_const_rem_by_zero
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t17944094","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393598768,"raw_hex":"e81ece1101","size":5,"soft_fail":false,"target_va":5411543584,"va":5393599485,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<image::dynimage::DynamicImage> >
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t-30240","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393598768,"raw_hex":"e8e089ffff","size":5,"soft_fail":false,"target_va":5393569104,"va":5393599339,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<image::error::ImageError> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-31157","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393598768,"raw_hex":"e84b86ffff","size":5,"soft_fail":false,"target_va":5393568528,"va":5393599680,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t17943404","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393598768,"raw_hex":"e86ccb1101","size":5,"soft_fail":false,"target_va":5411542880,"va":5393599471,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] image::dynimage::DynamicImage::new_rgba8
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"image","version":"0.24.9","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12644850","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393598768,"raw_hex":"e80e0e3fff","size":5,"soft_fail":false,"target_va":5380954144,"va":5393598989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] image::dynimage::DynamicImage::resize_exact
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"image","version":"0.24.9","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12644040","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393598768,"raw_hex":"e838113fff","size":5,"soft_fail":false,"target_va":5380954832,"va":5393598867,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] asset_pipe::texture::pad_image
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":1727,"record_offset":52068,"parent_record_offset":51760,"ancestor_record_offset":51760,"inlinee":77760,"source_line":306,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] asset_pipe::texture::resize_image
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":1727,"record_offset":51968,"parent_record_offset":51760,"ancestor_record_offset":51760,"inlinee":77759,"source_line":247,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1727,"record_offset":51912,"parent_record_offset":51760,"ancestor_record_offset":51760,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$8::next_multiple_of
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1727,"record_offset":51996,"parent_record_offset":51760,"ancestor_record_offset":51760,"inlinee":77758,"source_line":3233,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture::resize_image
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":1727,"record_offset":52164,"parent_record_offset":52068,"ancestor_record_offset":51760,"inlinee":77759,"source_line":247,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$64::lt
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1727,"record_offset":52292,"parent_record_offset":52260,"ancestor_record_offset":51760,"inlinee":5290,"source_line":1824,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$39::forward_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1727,"record_offset":52324,"parent_record_offset":52260,"ancestor_record_offset":51760,"inlinee":5292,"source_line":203,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1727,"record_offset":52260,"parent_record_offset":52232,"ancestor_record_offset":51760,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1727,"record_offset":52232,"parent_record_offset":52192,"ancestor_record_offset":51760,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::unchecked_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1727,"record_offset":52348,"parent_record_offset":52324,"ancestor_record_offset":51760,"inlinee":5293,"source_line":560,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1727,"record_offset":52932,"parent_record_offset":52068,"ancestor_record_offset":51760,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] image::image::GenericImage::copy_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/image.rs
//     evidence={"module":1727,"record_offset":52192,"parent_record_offset":52068,"ancestor_record_offset":51760,"inlinee":77762,"source_line":963,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::process_regular_image::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 440
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A6D8
// 首条位置: {"module":3187,"inlinee":108248}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3187,"record_offset":3120,"parent_record_offset":2012,"ancestor_record_offset":2012,"inlinee":5425,"source_line":3033,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$19::extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3187,"record_offset":2096,"parent_record_offset":2012,"ancestor_record_offset":2012,"inlinee":4917,"source_line":3492,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3187,"record_offset":2060,"parent_record_offset":2012,"ancestor_record_offset":2012,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_direct] image::buffer_::impl$41::deref
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":3187,"record_offset":3092,"parent_record_offset":2012,"ancestor_record_offset":2012,"inlinee":9524,"source_line":1110,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3187,"record_offset":2740,"parent_record_offset":2716,"ancestor_record_offset":2012,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3187,"record_offset":2716,"parent_record_offset":2660,"ancestor_record_offset":2012,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3187,"record_offset":2296,"parent_record_offset":2268,"ancestor_record_offset":2012,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3187,"record_offset":2392,"parent_record_offset":2364,"ancestor_record_offset":2012,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3187,"record_offset":2688,"parent_record_offset":2660,"ancestor_record_offset":2012,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3187,"record_offset":2660,"parent_record_offset":2636,"ancestor_record_offset":2012,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3187,"record_offset":2364,"parent_record_offset":2328,"ancestor_record_offset":2012,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3187,"record_offset":2328,"parent_record_offset":2296,"ancestor_record_offset":2012,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3187,"record_offset":2636,"parent_record_offset":2612,"ancestor_record_offset":2012,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3187,"record_offset":2224,"parent_record_offset":2184,"ancestor_record_offset":2012,"inlinee":5428,"source_line":2565,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3187,"record_offset":2484,"parent_record_offset":2224,"ancestor_record_offset":2012,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3187,"record_offset":2268,"parent_record_offset":2224,"ancestor_record_offset":2012,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3187,"record_offset":2128,"parent_record_offset":2096,"ancestor_record_offset":2012,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":3187,"record_offset":2540,"parent_record_offset":2516,"ancestor_record_offset":2012,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":3187,"record_offset":2588,"parent_record_offset":2564,"ancestor_record_offset":2012,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$2::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":3187,"record_offset":2184,"parent_record_offset":2096,"ancestor_record_offset":2012,"inlinee":8579,"source_line":35,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":3187,"record_offset":3148,"parent_record_offset":3120,"ancestor_record_offset":2012,"inlinee":5427,"source_line":59,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\manually_drop.rs
//     evidence={"module":3187,"record_offset":2152,"parent_record_offset":2128,"ancestor_record_offset":2012,"inlinee":5075,"source_line":180,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3187,"record_offset":2516,"parent_record_offset":2184,"ancestor_record_offset":2012,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3187,"record_offset":2448,"parent_record_offset":2224,"ancestor_record_offset":2012,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::with_rgba8<asset_pipe::texture::process_regular_image::closure_env$2,alloc::vec::Vec<u8,alloc::alloc::Global> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 485
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3187,"record_offset":5752,"rva":34484352,"va":5403193472}
// 编译实例 RVAs: 0x20e3080
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28112384","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403193472,"raw_hex":"e8000a53fe","size":5,"soft_fail":false,"target_va":5375081488,"va":5403193867,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28112529","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403193472,"raw_hex":"e86f0953fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5403194028,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8653203","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403193472,"raw_hex":"e893098400","size":5,"soft_fail":false,"target_va":5411847168,"va":5403193960,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t8346827","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403193472,"raw_hex":"e8cb5c7f00","size":5,"soft_fail":false,"target_va":5411540963,"va":5403194131,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] asset_pipe::texture::compress
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9593906","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403193472,"raw_hex":"e8ce9b6dff","size":5,"soft_fail":false,"target_va":5393599840,"va":5403193741,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] asset_pipe::texture::srgb_oetf
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9593822","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403193472,"raw_hex":"e8229c6dff","size":5,"soft_fail":false,"target_va":5393599776,"va":5403193593,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t8347687","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403193472,"raw_hex":"e827607f00","size":5,"soft_fail":false,"target_va":5411541776,"va":5403194084,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8349033","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403193472,"raw_hex":"e869657f00","size":5,"soft_fail":false,"target_va":5411543104,"va":5403194066,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] image::dynimage::DynamicImage::into_rgba8
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"image","version":"0.24.9","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-22239099","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403193472,"raw_hex":"e885a8acfe","size":5,"soft_fail":false,"target_va":5380954608,"va":5403193702,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] image::dynimage::DynamicImage::to_rgba32f
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"image","version":"0.24.9","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-22239067","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403193472,"raw_hex":"e8a5a8acfe","size":5,"soft_fail":false,"target_va":5380954464,"va":5403193526,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] asset_pipe::texture::apply_transfer_function
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":3187,"record_offset":5988,"parent_record_offset":5752,"ancestor_record_offset":5752,"inlinee":77751,"source_line":504,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_direct] asset_pipe::texture::process_regular_image::closure$2
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":3187,"record_offset":6580,"parent_record_offset":5752,"ancestor_record_offset":5752,"inlinee":108250,"source_line":415,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3187,"record_offset":7548,"parent_record_offset":5752,"ancestor_record_offset":5752,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3187,"record_offset":7268,"parent_record_offset":5752,"ancestor_record_offset":5752,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_direct] image::dynimage::DynamicImage::as_rgba8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/dynimage.rs
//     evidence={"module":3187,"record_offset":7520,"parent_record_offset":5752,"ancestor_record_offset":5752,"inlinee":108249,"source_line":391,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_direct] image::dynimage::impl$10::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/dynimage.rs
//     evidence={"module":3187,"record_offset":6552,"parent_record_offset":5752,"ancestor_record_offset":5752,"inlinee":77755,"source_line":929,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3187,"record_offset":7116,"parent_record_offset":7092,"ancestor_record_offset":5752,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3187,"record_offset":7140,"parent_record_offset":7116,"ancestor_record_offset":5752,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3187,"record_offset":7468,"parent_record_offset":7444,"ancestor_record_offset":5752,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3187,"record_offset":7092,"parent_record_offset":6916,"ancestor_record_offset":5752,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3187,"record_offset":7444,"parent_record_offset":7380,"ancestor_record_offset":5752,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3187,"record_offset":6856,"parent_record_offset":6828,"ancestor_record_offset":5752,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3187,"record_offset":7412,"parent_record_offset":7380,"ancestor_record_offset":5752,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3187,"record_offset":7380,"parent_record_offset":7352,"ancestor_record_offset":5752,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3187,"record_offset":6916,"parent_record_offset":6884,"ancestor_record_offset":5752,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3187,"record_offset":6884,"parent_record_offset":6856,"ancestor_record_offset":5752,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3187,"record_offset":7352,"parent_record_offset":7324,"ancestor_record_offset":5752,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3187,"record_offset":6952,"parent_record_offset":6916,"ancestor_record_offset":5752,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3187,"record_offset":6788,"parent_record_offset":6756,"ancestor_record_offset":5752,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3187,"record_offset":6756,"parent_record_offset":6724,"ancestor_record_offset":5752,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3187,"record_offset":6692,"parent_record_offset":6580,"ancestor_record_offset":5752,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3187,"record_offset":6724,"parent_record_offset":6692,"ancestor_record_offset":5752,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3187,"record_offset":6828,"parent_record_offset":6788,"ancestor_record_offset":5752,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":3187,"record_offset":6976,"parent_record_offset":6952,"ancestor_record_offset":5752,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":3187,"record_offset":7000,"parent_record_offset":6976,"ancestor_record_offset":5752,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":3187,"record_offset":7216,"parent_record_offset":7192,"ancestor_record_offset":5752,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":3187,"record_offset":7048,"parent_record_offset":7024,"ancestor_record_offset":5752,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3187,"record_offset":6164,"parent_record_offset":6144,"ancestor_record_offset":5752,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3187,"record_offset":6188,"parent_record_offset":6164,"ancestor_record_offset":5752,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3187,"record_offset":8656,"parent_record_offset":8624,"ancestor_record_offset":5752,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3187,"record_offset":6120,"parent_record_offset":6096,"ancestor_record_offset":5752,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3187,"record_offset":6340,"parent_record_offset":6056,"ancestor_record_offset":5752,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3187,"record_offset":7192,"parent_record_offset":6788,"ancestor_record_offset":5752,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3187,"record_offset":7296,"parent_record_offset":7268,"ancestor_record_offset":5752,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3187,"record_offset":6376,"parent_record_offset":6056,"ancestor_record_offset":5752,"inlinee":5238,"source_line":26,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3187,"record_offset":6432,"parent_record_offset":6404,"ancestor_record_offset":5752,"inlinee":5308,"source_line":444,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3187,"record_offset":6404,"parent_record_offset":6376,"ancestor_record_offset":5752,"inlinee":5312,"source_line":526,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$98::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":3187,"record_offset":6512,"parent_record_offset":6484,"ancestor_record_offset":5752,"inlinee":9507,"source_line":2069,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::image_buffer_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":3187,"record_offset":6096,"parent_record_offset":6056,"ancestor_record_offset":5752,"inlinee":7815,"source_line":807,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::inner_pixels_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":3187,"record_offset":6056,"parent_record_offset":6024,"ancestor_record_offset":5752,"inlinee":77752,"source_line":892,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::pixels_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":3187,"record_offset":6024,"parent_record_offset":5988,"ancestor_record_offset":5752,"inlinee":9499,"source_line":898,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$34::image_buffer_len::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":3187,"record_offset":6144,"parent_record_offset":6120,"ancestor_record_offset":5752,"inlinee":7817,"source_line":809,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$34::image_buffer_len::closure$1
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":3187,"record_offset":6252,"parent_record_offset":6228,"ancestor_record_offset":5752,"inlinee":7818,"source_line":810,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$41::deref
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":3187,"record_offset":6664,"parent_record_offset":6580,"ancestor_record_offset":5752,"inlinee":9524,"source_line":1110,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$5::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":3187,"record_offset":6484,"parent_record_offset":5988,"ancestor_record_offset":5752,"inlinee":9505,"source_line":98,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::with_rgba8<asset_pipe::texture::process_regular_image::closure_env$4,tuple$<> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 485
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3187,"record_offset":1204,"rva":34483152,"va":5403192272}
// 编译实例 RVAs: 0x20e2bd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28111123","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403192272,"raw_hex":"e8ed0e53fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5403192622,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8654574","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403192272,"raw_hex":"e8ee0e8400","size":5,"soft_fail":false,"target_va":5411847168,"va":5403192589,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t8598537","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403192272,"raw_hex":"e809348300","size":5,"soft_fail":false,"target_va":5411791504,"va":5403192962,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] asset_pipe::texture::compress
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9592705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403192272,"raw_hex":"e87fa06dff","size":5,"soft_fail":false,"target_va":5393599840,"va":5403192540,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] asset_pipe::texture::srgb_oetf
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9592621","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403192272,"raw_hex":"e8d3a06dff","size":5,"soft_fail":false,"target_va":5393599776,"va":5403192392,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t8348638","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403192272,"raw_hex":"e8de637f00","size":5,"soft_fail":false,"target_va":5411541776,"va":5403193133,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8350171","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403192272,"raw_hex":"e8db697f00","size":5,"soft_fail":false,"target_va":5411543104,"va":5403192928,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] image::dynimage::DynamicImage::into_rgba8
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"image","version":"0.24.9","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-22237897","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403192272,"raw_hex":"e837adacfe","size":5,"soft_fail":false,"target_va":5380954608,"va":5403192500,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] image::dynimage::DynamicImage::to_rgba32f
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"image","version":"0.24.9","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-22237861","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403192272,"raw_hex":"e85badacfe","size":5,"soft_fail":false,"target_va":5380954464,"va":5403192320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] asset_pipe::texture::apply_transfer_function
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":3187,"record_offset":1408,"parent_record_offset":1204,"ancestor_record_offset":1204,"inlinee":77751,"source_line":504,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_direct] asset_pipe::texture::process_regular_image::closure$4
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":3187,"record_offset":2012,"parent_record_offset":1204,"ancestor_record_offset":1204,"inlinee":108248,"source_line":440,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3187,"record_offset":3508,"parent_record_offset":1204,"ancestor_record_offset":1204,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3187,"record_offset":5028,"parent_record_offset":1204,"ancestor_record_offset":1204,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_direct] image::dynimage::DynamicImage::as_rgba8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/dynimage.rs
//     evidence={"module":3187,"record_offset":3480,"parent_record_offset":1204,"ancestor_record_offset":1204,"inlinee":108249,"source_line":391,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_direct] image::dynimage::impl$10::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/dynimage.rs
//     evidence={"module":3187,"record_offset":1984,"parent_record_offset":1204,"ancestor_record_offset":1204,"inlinee":77755,"source_line":929,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3187,"record_offset":2740,"parent_record_offset":2716,"ancestor_record_offset":1204,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3187,"record_offset":2716,"parent_record_offset":2660,"ancestor_record_offset":1204,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3187,"record_offset":2296,"parent_record_offset":2268,"ancestor_record_offset":1204,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3187,"record_offset":2392,"parent_record_offset":2364,"ancestor_record_offset":1204,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3187,"record_offset":2688,"parent_record_offset":2660,"ancestor_record_offset":1204,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3187,"record_offset":2660,"parent_record_offset":2636,"ancestor_record_offset":1204,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3187,"record_offset":2364,"parent_record_offset":2328,"ancestor_record_offset":1204,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3187,"record_offset":2328,"parent_record_offset":2296,"ancestor_record_offset":1204,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3187,"record_offset":2636,"parent_record_offset":2612,"ancestor_record_offset":1204,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3187,"record_offset":2224,"parent_record_offset":2184,"ancestor_record_offset":1204,"inlinee":5428,"source_line":2565,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3187,"record_offset":3120,"parent_record_offset":2012,"ancestor_record_offset":1204,"inlinee":5425,"source_line":3033,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3187,"record_offset":2484,"parent_record_offset":2224,"ancestor_record_offset":1204,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3187,"record_offset":2268,"parent_record_offset":2224,"ancestor_record_offset":1204,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3187,"record_offset":2128,"parent_record_offset":2096,"ancestor_record_offset":1204,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3187,"record_offset":2096,"parent_record_offset":2012,"ancestor_record_offset":1204,"inlinee":4917,"source_line":3492,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":3187,"record_offset":2540,"parent_record_offset":2516,"ancestor_record_offset":1204,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":3187,"record_offset":2588,"parent_record_offset":2564,"ancestor_record_offset":1204,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$2::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":3187,"record_offset":2184,"parent_record_offset":2096,"ancestor_record_offset":1204,"inlinee":8579,"source_line":35,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":3187,"record_offset":3148,"parent_record_offset":3120,"ancestor_record_offset":1204,"inlinee":5427,"source_line":59,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\manually_drop.rs
//     evidence={"module":3187,"record_offset":2152,"parent_record_offset":2128,"ancestor_record_offset":1204,"inlinee":5075,"source_line":180,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3187,"record_offset":1596,"parent_record_offset":1576,"ancestor_record_offset":1204,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3187,"record_offset":1620,"parent_record_offset":1596,"ancestor_record_offset":1204,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3187,"record_offset":5640,"parent_record_offset":5608,"ancestor_record_offset":1204,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3187,"record_offset":1552,"parent_record_offset":1528,"ancestor_record_offset":1204,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3187,"record_offset":1772,"parent_record_offset":1484,"ancestor_record_offset":1204,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3187,"record_offset":2516,"parent_record_offset":2184,"ancestor_record_offset":1204,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3187,"record_offset":2448,"parent_record_offset":2224,"ancestor_record_offset":1204,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3187,"record_offset":1808,"parent_record_offset":1484,"ancestor_record_offset":1204,"inlinee":5238,"source_line":26,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3187,"record_offset":1864,"parent_record_offset":1836,"ancestor_record_offset":1204,"inlinee":5308,"source_line":444,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3187,"record_offset":1836,"parent_record_offset":1808,"ancestor_record_offset":1204,"inlinee":5312,"source_line":526,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$98::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":3187,"record_offset":1944,"parent_record_offset":1916,"ancestor_record_offset":1204,"inlinee":9507,"source_line":2069,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::image_buffer_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":3187,"record_offset":1528,"parent_record_offset":1484,"ancestor_record_offset":1204,"inlinee":7815,"source_line":807,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::inner_pixels_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":3187,"record_offset":1484,"parent_record_offset":1448,"ancestor_record_offset":1204,"inlinee":77752,"source_line":892,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::pixels_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":3187,"record_offset":1448,"parent_record_offset":1408,"ancestor_record_offset":1204,"inlinee":9499,"source_line":898,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$34::image_buffer_len::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":3187,"record_offset":1576,"parent_record_offset":1552,"ancestor_record_offset":1204,"inlinee":7817,"source_line":809,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$34::image_buffer_len::closure$1
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":3187,"record_offset":1684,"parent_record_offset":1660,"ancestor_record_offset":1204,"inlinee":7818,"source_line":810,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$41::deref
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":3187,"record_offset":3092,"parent_record_offset":2012,"ancestor_record_offset":1204,"inlinee":9524,"source_line":1110,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$5::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":3187,"record_offset":1916,"parent_record_offset":1408,"ancestor_record_offset":1204,"inlinee":9505,"source_line":98,"archive":"_pdb/B/modules/03187.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::apply_transfer_function
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 504
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12FB7
// 首条位置: {"module":1727,"inlinee":77751}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset_pipe::texture::srgb_eotf
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":1727,"record_offset":49752,"parent_record_offset":49164,"ancestor_record_offset":49164,"inlinee":77753,"source_line":524,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] image::buffer_::ImageBuffer::pixels_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":1727,"record_offset":49208,"parent_record_offset":49164,"ancestor_record_offset":49164,"inlinee":9499,"source_line":898,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] image::buffer_::impl$5::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":1727,"record_offset":49680,"parent_record_offset":49164,"ancestor_record_offset":49164,"inlinee":9505,"source_line":98,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1727,"record_offset":49356,"parent_record_offset":49332,"ancestor_record_offset":49164,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1727,"record_offset":49380,"parent_record_offset":49356,"ancestor_record_offset":49164,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1727,"record_offset":49308,"parent_record_offset":49280,"ancestor_record_offset":49164,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1727,"record_offset":49536,"parent_record_offset":49240,"ancestor_record_offset":49164,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1727,"record_offset":49572,"parent_record_offset":49240,"ancestor_record_offset":49164,"inlinee":5238,"source_line":26,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1727,"record_offset":49628,"parent_record_offset":49600,"ancestor_record_offset":49164,"inlinee":5308,"source_line":444,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::index_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1727,"record_offset":49600,"parent_record_offset":49572,"ancestor_record_offset":49164,"inlinee":5312,"source_line":526,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$98::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1727,"record_offset":49712,"parent_record_offset":49680,"ancestor_record_offset":49164,"inlinee":9507,"source_line":2069,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::image_buffer_len
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":1727,"record_offset":49280,"parent_record_offset":49240,"ancestor_record_offset":49164,"inlinee":7815,"source_line":807,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::inner_pixels_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":1727,"record_offset":49240,"parent_record_offset":49208,"ancestor_record_offset":49164,"inlinee":77752,"source_line":892,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$34::image_buffer_len::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":1727,"record_offset":49332,"parent_record_offset":49308,"ancestor_record_offset":49164,"inlinee":7817,"source_line":809,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$34::image_buffer_len::closure$1
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":1727,"record_offset":49444,"parent_record_offset":49420,"ancestor_record_offset":49164,"inlinee":7818,"source_line":810,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::powf
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1727,"record_offset":49788,"parent_record_offset":49752,"ancestor_record_offset":49164,"inlinee":11915,"source_line":339,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::srgb_oetf
// 记录形式: compiled_procedure; instances=1
// 源码行: 516
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1727,"record_offset":52972,"rva":24890656,"va":5393599776}
// 编译实例 RVAs: 0x17bcd20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142923f70
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18248242","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393599776,"raw_hex":"e832721601","size":5,"soft_fail":false,"target_va":5411848048,"va":5393599801,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] std::f32::impl$0::powf
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1727,"record_offset":53088,"parent_record_offset":52972,"ancestor_record_offset":52972,"inlinee":11915,"source_line":339,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::srgb_eotf
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 524
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12FB9
// 首条位置: {"module":1727,"inlinee":77753}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::f32::impl$0::powf
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1727,"record_offset":49788,"parent_record_offset":49752,"ancestor_record_offset":49752,"inlinee":11915,"source_line":339,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::process_dds_image
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 532
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12FAE
// 首条位置: {"module":1727,"inlinee":77742}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1727,"record_offset":42648,"parent_record_offset":40428,"ancestor_record_offset":40428,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1727,"record_offset":41520,"parent_record_offset":40428,"ancestor_record_offset":40428,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$6::try_into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1727,"record_offset":42556,"parent_record_offset":40428,"ancestor_record_offset":40428,"inlinee":9618,"source_line":799,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1727,"record_offset":42528,"parent_record_offset":40428,"ancestor_record_offset":40428,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1727,"record_offset":44380,"parent_record_offset":40428,"ancestor_record_offset":40428,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1727,"record_offset":41696,"parent_record_offset":40428,"ancestor_record_offset":40428,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1727,"record_offset":40980,"parent_record_offset":40428,"ancestor_record_offset":40428,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1727,"record_offset":42360,"parent_record_offset":40428,"ancestor_record_offset":40428,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1727,"record_offset":40880,"parent_record_offset":40428,"ancestor_record_offset":40428,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1727,"record_offset":43868,"parent_record_offset":40428,"ancestor_record_offset":40428,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] ddsfile::Dds::get_depth
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ddsfile","version":"0.5.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ddsfile-0.5.1/src/lib.rs
//     evidence={"module":1727,"record_offset":41552,"parent_record_offset":40428,"ancestor_record_offset":40428,"inlinee":77746,"source_line":249,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] ddsfile::Dds::get_dxgi_format
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ddsfile","version":"0.5.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ddsfile-0.5.1/src/lib.rs
//     evidence={"module":1727,"record_offset":41608,"parent_record_offset":40428,"ancestor_record_offset":40428,"inlinee":77747,"source_line":218,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1727,"record_offset":41956,"parent_record_offset":40428,"ancestor_record_offset":40428,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1727,"record_offset":41436,"parent_record_offset":40428,"ancestor_record_offset":40428,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1727,"record_offset":40656,"parent_record_offset":40428,"ancestor_record_offset":40428,"inlinee":12456,"source_line":267,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] std::io::cursor::Cursor::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\cursor.rs
//     evidence={"module":1727,"record_offset":40912,"parent_record_offset":40428,"ancestor_record_offset":40428,"inlinee":21628,"source_line":98,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1727,"record_offset":43224,"parent_record_offset":43200,"ancestor_record_offset":40428,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1727,"record_offset":43248,"parent_record_offset":43224,"ancestor_record_offset":40428,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1727,"record_offset":41156,"parent_record_offset":41132,"ancestor_record_offset":40428,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1727,"record_offset":43200,"parent_record_offset":43008,"ancestor_record_offset":40428,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1727,"record_offset":41132,"parent_record_offset":41076,"ancestor_record_offset":40428,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1727,"record_offset":41752,"parent_record_offset":41728,"ancestor_record_offset":40428,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1727,"record_offset":41800,"parent_record_offset":41776,"ancestor_record_offset":40428,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1727,"record_offset":42916,"parent_record_offset":42872,"ancestor_record_offset":40428,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1727,"record_offset":41104,"parent_record_offset":41076,"ancestor_record_offset":40428,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1727,"record_offset":41076,"parent_record_offset":41052,"ancestor_record_offset":40428,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1727,"record_offset":43008,"parent_record_offset":42960,"ancestor_record_offset":40428,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1727,"record_offset":42960,"parent_record_offset":42916,"ancestor_record_offset":40428,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1727,"record_offset":41052,"parent_record_offset":41028,"ancestor_record_offset":40428,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1727,"record_offset":43060,"parent_record_offset":43008,"ancestor_record_offset":40428,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1727,"record_offset":42824,"parent_record_offset":42780,"ancestor_record_offset":40428,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1727,"record_offset":42780,"parent_record_offset":42736,"ancestor_record_offset":40428,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1727,"record_offset":42692,"parent_record_offset":42648,"ancestor_record_offset":40428,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1727,"record_offset":42736,"parent_record_offset":42692,"ancestor_record_offset":40428,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1727,"record_offset":42872,"parent_record_offset":42824,"ancestor_record_offset":40428,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture::process_dds_image::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":1727,"record_offset":41728,"parent_record_offset":41696,"ancestor_record_offset":40428,"inlinee":77749,"source_line":544,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture::process_dds_image::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":1727,"record_offset":41840,"parent_record_offset":41696,"ancestor_record_offset":40428,"inlinee":77750,"source_line":545,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1727,"record_offset":43084,"parent_record_offset":43060,"ancestor_record_offset":40428,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1727,"record_offset":43108,"parent_record_offset":43084,"ancestor_record_offset":40428,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1727,"record_offset":40680,"parent_record_offset":40656,"ancestor_record_offset":40428,"inlinee":5126,"source_line":703,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$118::try_from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\num.rs
//     evidence={"module":1727,"record_offset":42580,"parent_record_offset":42556,"ancestor_record_offset":40428,"inlinee":12127,"source_line":255,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1727,"record_offset":43324,"parent_record_offset":43300,"ancestor_record_offset":40428,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1727,"record_offset":43156,"parent_record_offset":43132,"ancestor_record_offset":40428,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1727,"record_offset":43132,"parent_record_offset":43108,"ancestor_record_offset":40428,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1727,"record_offset":41776,"parent_record_offset":41752,"ancestor_record_offset":40428,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1727,"record_offset":41576,"parent_record_offset":41552,"ancestor_record_offset":40428,"inlinee":4666,"source_line":998,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1727,"record_offset":42012,"parent_record_offset":41980,"ancestor_record_offset":40428,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1727,"record_offset":42040,"parent_record_offset":41980,"ancestor_record_offset":40428,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1727,"record_offset":43300,"parent_record_offset":42824,"ancestor_record_offset":40428,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1727,"record_offset":41004,"parent_record_offset":40980,"ancestor_record_offset":40428,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1727,"record_offset":41460,"parent_record_offset":41436,"ancestor_record_offset":40428,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1727,"record_offset":41484,"parent_record_offset":41460,"ancestor_record_offset":40428,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] ddsfile::format::dxgi::DxgiFormat::try_from_pixel_format
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ddsfile","version":"0.5.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ddsfile-0.5.1/src/format/dxgi.rs
//     evidence={"module":1727,"record_offset":41644,"parent_record_offset":41608,"ancestor_record_offset":40428,"inlinee":77709,"source_line":416,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1727,"record_offset":42068,"parent_record_offset":41980,"ancestor_record_offset":40428,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1727,"record_offset":41980,"parent_record_offset":41956,"ancestor_record_offset":40428,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1727,"record_offset":42092,"parent_record_offset":42068,"ancestor_record_offset":40428,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1727,"record_offset":40776,"parent_record_offset":40752,"ancestor_record_offset":40428,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1727,"record_offset":40824,"parent_record_offset":40800,"ancestor_record_offset":40428,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1727,"record_offset":40800,"parent_record_offset":40776,"ancestor_record_offset":40428,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1727,"record_offset":40752,"parent_record_offset":40728,"ancestor_record_offset":40428,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1727,"record_offset":40728,"parent_record_offset":40704,"ancestor_record_offset":40428,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1727,"record_offset":40704,"parent_record_offset":40680,"ancestor_record_offset":40428,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::process_dds_image::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 534
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A654
// 首条位置: {"module":3165,"inlinee":108116}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":3165,"record_offset":4828,"parent_record_offset":4808,"ancestor_record_offset":4808,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/03165.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":3165,"record_offset":4876,"parent_record_offset":4852,"ancestor_record_offset":4808,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/03165.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3165,"record_offset":4852,"parent_record_offset":4828,"ancestor_record_offset":4808,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/03165.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::process_dds_image::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 536
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A653
// 首条位置: {"module":3165,"inlinee":108115}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":3165,"record_offset":4152,"parent_record_offset":4132,"ancestor_record_offset":4132,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/03165.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":3165,"record_offset":4200,"parent_record_offset":4176,"ancestor_record_offset":4132,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/03165.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3165,"record_offset":4176,"parent_record_offset":4152,"ancestor_record_offset":4132,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/03165.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::process_dds_image::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 544
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12FB5
// 首条位置: {"module":1727,"inlinee":77749}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1727,"record_offset":41752,"parent_record_offset":41728,"ancestor_record_offset":41728,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1727,"record_offset":41800,"parent_record_offset":41776,"ancestor_record_offset":41728,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1727,"record_offset":41776,"parent_record_offset":41752,"ancestor_record_offset":41728,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::process_dds_image::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 545
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12FB6
// 首条位置: {"module":1727,"inlinee":77750}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1727,"record_offset":41864,"parent_record_offset":41840,"ancestor_record_offset":41840,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1727,"record_offset":41912,"parent_record_offset":41888,"ancestor_record_offset":41840,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1727,"record_offset":41888,"parent_record_offset":41864,"ancestor_record_offset":41840,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::process_image_array
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 596
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12FA1
// 首条位置: {"module":1727,"inlinee":77729}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1727,"record_offset":37320,"parent_record_offset":33880,"ancestor_record_offset":33880,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$19::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1727,"record_offset":35992,"parent_record_offset":33880,"ancestor_record_offset":33880,"inlinee":4917,"source_line":3492,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::array::equality::impl$0::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\equality.rs
//     evidence={"module":1727,"record_offset":35596,"parent_record_offset":33880,"ancestor_record_offset":33880,"inlinee":14532,"source_line":13,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1727,"record_offset":35680,"parent_record_offset":33880,"ancestor_record_offset":33880,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1727,"record_offset":34200,"parent_record_offset":33880,"ancestor_record_offset":33880,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1727,"record_offset":34540,"parent_record_offset":33880,"ancestor_record_offset":33880,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1727,"record_offset":37284,"parent_record_offset":33880,"ancestor_record_offset":33880,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1727,"record_offset":36932,"parent_record_offset":33880,"ancestor_record_offset":33880,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1727,"record_offset":34420,"parent_record_offset":33880,"ancestor_record_offset":33880,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1727,"record_offset":34304,"parent_record_offset":33880,"ancestor_record_offset":33880,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1727,"record_offset":38372,"parent_record_offset":33880,"ancestor_record_offset":33880,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1727,"record_offset":34116,"parent_record_offset":33880,"ancestor_record_offset":33880,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] rkyv::util::archived_root
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.7.39","source":"git+https://github.com/PounceLight/rkyv.git?rev=031d58108ccf8c0e125b06b017f72b4afd67e9a3#031d58108ccf8c0e125b06b017f72b4afd67e9a3"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-79c0f5fde0331f7b/031d581/rkyv/src/util/mod.rs
//     evidence={"module":1727,"record_offset":34336,"parent_record_offset":33880,"ancestor_record_offset":33880,"inlinee":77731,"source_line":147,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] rkyv::vec::impl$4::deref
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.7.39","source":"git+https://github.com/PounceLight/rkyv.git?rev=031d58108ccf8c0e125b06b017f72b4afd67e9a3#031d58108ccf8c0e125b06b017f72b4afd67e9a3"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-79c0f5fde0331f7b/031d581/rkyv/src/vec/mod.rs
//     evidence={"module":1727,"record_offset":34456,"parent_record_offset":33880,"ancestor_record_offset":33880,"inlinee":77734,"source_line":207,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1727,"record_offset":34968,"parent_record_offset":34944,"ancestor_record_offset":33880,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1727,"record_offset":34992,"parent_record_offset":34968,"ancestor_record_offset":33880,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1727,"record_offset":37132,"parent_record_offset":37108,"ancestor_record_offset":33880,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1727,"record_offset":34944,"parent_record_offset":34904,"ancestor_record_offset":33880,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1727,"record_offset":37108,"parent_record_offset":37052,"ancestor_record_offset":33880,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1727,"record_offset":37368,"parent_record_offset":37344,"ancestor_record_offset":33880,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1727,"record_offset":36140,"parent_record_offset":36112,"ancestor_record_offset":33880,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1727,"record_offset":34824,"parent_record_offset":34784,"ancestor_record_offset":33880,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1727,"record_offset":37080,"parent_record_offset":37052,"ancestor_record_offset":33880,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1727,"record_offset":37052,"parent_record_offset":37028,"ancestor_record_offset":33880,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1727,"record_offset":36200,"parent_record_offset":36168,"ancestor_record_offset":33880,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1727,"record_offset":36168,"parent_record_offset":36140,"ancestor_record_offset":33880,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1727,"record_offset":34904,"parent_record_offset":34864,"ancestor_record_offset":33880,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1727,"record_offset":34864,"parent_record_offset":34824,"ancestor_record_offset":33880,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1727,"record_offset":37028,"parent_record_offset":37004,"ancestor_record_offset":33880,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1727,"record_offset":35088,"parent_record_offset":35048,"ancestor_record_offset":33880,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1727,"record_offset":36112,"parent_record_offset":36064,"ancestor_record_offset":33880,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1727,"record_offset":34744,"parent_record_offset":34684,"ancestor_record_offset":33880,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1727,"record_offset":34784,"parent_record_offset":34744,"ancestor_record_offset":33880,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1727,"record_offset":34588,"parent_record_offset":34540,"ancestor_record_offset":33880,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1727,"record_offset":35372,"parent_record_offset":35324,"ancestor_record_offset":33880,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1727,"record_offset":35484,"parent_record_offset":35372,"ancestor_record_offset":33880,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1727,"record_offset":36892,"parent_record_offset":36064,"ancestor_record_offset":33880,"inlinee":6531,"source_line":13,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1727,"record_offset":36832,"parent_record_offset":36804,"ancestor_record_offset":33880,"inlinee":6529,"source_line":30,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1727,"record_offset":35048,"parent_record_offset":34684,"ancestor_record_offset":33880,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":1727,"record_offset":34636,"parent_record_offset":34588,"ancestor_record_offset":33880,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":1727,"record_offset":34684,"parent_record_offset":34636,"ancestor_record_offset":33880,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::blob::impl$3::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/blob.rs
//     evidence={"module":1727,"record_offset":35704,"parent_record_offset":35680,"ancestor_record_offset":33880,"inlinee":77737,"source_line":5,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::blob::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/blob.rs
//     evidence={"module":1727,"record_offset":35760,"parent_record_offset":35736,"ancestor_record_offset":33880,"inlinee":77739,"source_line":196,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\equality.rs
//     evidence={"module":1727,"record_offset":35644,"parent_record_offset":35620,"ancestor_record_offset":33880,"inlinee":9478,"source_line":146,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\equality.rs
//     evidence={"module":1727,"record_offset":35620,"parent_record_offset":35596,"ancestor_record_offset":33880,"inlinee":9488,"source_line":152,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::copy_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":1727,"record_offset":35276,"parent_record_offset":35208,"ancestor_record_offset":33880,"inlinee":9582,"source_line":30,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":1727,"record_offset":35168,"parent_record_offset":35128,"ancestor_record_offset":33880,"inlinee":9580,"source_line":71,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1727,"record_offset":35128,"parent_record_offset":35088,"ancestor_record_offset":33880,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1727,"record_offset":35324,"parent_record_offset":35276,"ancestor_record_offset":33880,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1727,"record_offset":36692,"parent_record_offset":36340,"ancestor_record_offset":33880,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1727,"record_offset":36224,"parent_record_offset":36200,"ancestor_record_offset":33880,"inlinee":5005,"source_line":2021,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1727,"record_offset":37344,"parent_record_offset":37320,"ancestor_record_offset":33880,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1727,"record_offset":34384,"parent_record_offset":34360,"ancestor_record_offset":33880,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1727,"record_offset":36720,"parent_record_offset":36340,"ancestor_record_offset":33880,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1727,"record_offset":34164,"parent_record_offset":34140,"ancestor_record_offset":33880,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1727,"record_offset":35528,"parent_record_offset":35208,"ancestor_record_offset":33880,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1727,"record_offset":34264,"parent_record_offset":34228,"ancestor_record_offset":33880,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1727,"record_offset":35432,"parent_record_offset":35372,"ancestor_record_offset":33880,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1727,"record_offset":34140,"parent_record_offset":34116,"ancestor_record_offset":33880,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1727,"record_offset":35208,"parent_record_offset":35168,"ancestor_record_offset":33880,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1727,"record_offset":34228,"parent_record_offset":34200,"ancestor_record_offset":33880,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::as_ptr
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.7.39","source":"git+https://github.com/PounceLight/rkyv.git?rev=031d58108ccf8c0e125b06b017f72b4afd67e9a3#031d58108ccf8c0e125b06b017f72b4afd67e9a3"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-79c0f5fde0331f7b/031d581/rkyv/src/rel_ptr/mod.rs
//     evidence={"module":1727,"record_offset":35892,"parent_record_offset":35868,"ancestor_record_offset":33880,"inlinee":28664,"source_line":212,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::offset
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.7.39","source":"git+https://github.com/PounceLight/rkyv.git?rev=031d58108ccf8c0e125b06b017f72b4afd67e9a3#031d58108ccf8c0e125b06b017f72b4afd67e9a3"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-79c0f5fde0331f7b/031d581/rkyv/src/rel_ptr/mod.rs
//     evidence={"module":1727,"record_offset":35916,"parent_record_offset":35892,"ancestor_record_offset":33880,"inlinee":28665,"source_line":200,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RelPtr::as_ptr
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.7.39","source":"git+https://github.com/PounceLight/rkyv.git?rev=031d58108ccf8c0e125b06b017f72b4afd67e9a3#031d58108ccf8c0e125b06b017f72b4afd67e9a3"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-79c0f5fde0331f7b/031d581/rkyv/src/rel_ptr/mod.rs
//     evidence={"module":1727,"record_offset":35868,"parent_record_offset":35840,"ancestor_record_offset":33880,"inlinee":28662,"source_line":418,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::impl$18::to_isize
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.7.39","source":"git+https://github.com/PounceLight/rkyv.git?rev=031d58108ccf8c0e125b06b017f72b4afd67e9a3#031d58108ccf8c0e125b06b017f72b4afd67e9a3"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-79c0f5fde0331f7b/031d581/rkyv/src/rel_ptr/mod.rs
//     evidence={"module":1727,"record_offset":35940,"parent_record_offset":35916,"ancestor_record_offset":33880,"inlinee":28667,"source_line":110,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::util::aligned_vec::impl$0::drop
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"rkyv","version":"0.7.39","source":"git+https://github.com/PounceLight/rkyv.git?rev=031d58108ccf8c0e125b06b017f72b4afd67e9a3#031d58108ccf8c0e125b06b017f72b4afd67e9a3"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-79c0f5fde0331f7b/031d581/rkyv/src/util/aligned_vec.rs
//     evidence={"module":1727,"record_offset":37216,"parent_record_offset":37188,"ancestor_record_offset":33880,"inlinee":77741,"source_line":22,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::util::archived_value
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.7.39","source":"git+https://github.com/PounceLight/rkyv.git?rev=031d58108ccf8c0e125b06b017f72b4afd67e9a3#031d58108ccf8c0e125b06b017f72b4afd67e9a3"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-79c0f5fde0331f7b/031d581/rkyv/src/util/mod.rs
//     evidence={"module":1727,"record_offset":34360,"parent_record_offset":34336,"ancestor_record_offset":33880,"inlinee":77732,"source_line":63,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::ArchivedVec::as_ptr
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.7.39","source":"git+https://github.com/PounceLight/rkyv.git?rev=031d58108ccf8c0e125b06b017f72b4afd67e9a3#031d58108ccf8c0e125b06b017f72b4afd67e9a3"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-79c0f5fde0331f7b/031d581/rkyv/src/vec/mod.rs
//     evidence={"module":1727,"record_offset":35840,"parent_record_offset":35816,"ancestor_record_offset":33880,"inlinee":28660,"source_line":32,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::ArchivedVec::as_slice
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.7.39","source":"git+https://github.com/PounceLight/rkyv.git?rev=031d58108ccf8c0e125b06b017f72b4afd67e9a3#031d58108ccf8c0e125b06b017f72b4afd67e9a3"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-79c0f5fde0331f7b/031d581/rkyv/src/vec/mod.rs
//     evidence={"module":1727,"record_offset":34480,"parent_record_offset":34456,"ancestor_record_offset":33880,"inlinee":28659,"source_line":50,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::ArchivedVec::len
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.7.39","source":"git+https://github.com/PounceLight/rkyv.git?rev=031d58108ccf8c0e125b06b017f72b4afd67e9a3#031d58108ccf8c0e125b06b017f72b4afd67e9a3"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-79c0f5fde0331f7b/031d581/rkyv/src/vec/mod.rs
//     evidence={"module":1727,"record_offset":34504,"parent_record_offset":34480,"ancestor_record_offset":33880,"inlinee":77735,"source_line":38,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::impl$2::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 682
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1727,"record_offset":56996,"rva":24892224,"va":5393601344}
// 编译实例 RVAs: 0x17bd340
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140d86b90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-10708946","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5393601344,"raw_hex":"e92e985cff","size":5,"soft_fail":false,"target_va":5382892432,"va":5393601373,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::BcMode::block_bytes
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 697
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12FC6
// 首条位置: {"module":1727,"inlinee":77766}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::compress
// 记录形式: compiled_procedure; instances=1
// 源码行: 705
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1727,"record_offset":53120,"rva":24890720,"va":5393599840}
// 编译实例 RVAs: 0x17bcd60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-10955763","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393599840,"raw_hex":"e80dd458ff","size":5,"soft_fail":false,"target_va":5382644528,"va":5393600286,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18519179","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393599840,"raw_hex":"e8756be5fe","size":5,"soft_fail":false,"target_va":5375081488,"va":5393600662,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18519472","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393599840,"raw_hex":"e8506ae5fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5393600971,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c40
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18518516","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393599840,"raw_hex":"e80c6ee5fe","size":5,"soft_fail":false,"target_va":5375081536,"va":5393600047,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18084126","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393599840,"raw_hex":"e8e20eecfe","size":5,"soft_fail":false,"target_va":5375516032,"va":5393600153,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420db9a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9562635","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393599840,"raw_hex":"e80bea9100","size":5,"soft_fail":false,"target_va":5403163040,"va":5393600400,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t17939894","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393599840,"raw_hex":"e8b6bd1101","size":5,"soft_fail":false,"target_va":5411540963,"va":5393601064,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t17940728","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393599840,"raw_hex":"e8f8c01101","size":5,"soft_fail":false,"target_va":5411541776,"va":5393601043,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t17942068","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393599840,"raw_hex":"e834c61101","size":5,"soft_fail":false,"target_va":5411543104,"va":5393601031,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] intel_tex_2::bc4::compress_blocks_into
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"intel_tex_2","version":"0.4.0","source":"git+https://github.com/PounceLight/intel-tex-rs-2.git?rev=4fb0e0b70f868bccd529d45e1626de2f55a20b83#4fb0e0b70f868bccd529d45e1626de2f55a20b83"}]
//     evidence={"assembly":"\tcallq\t9594219","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393599840,"raw_hex":"e86b659200","size":5,"soft_fail":false,"target_va":5403195168,"va":5393600944,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] intel_tex_2::bc5::compress_blocks_into
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"intel_tex_2","version":"0.4.0","source":"git+https://github.com/PounceLight/intel-tex-rs-2.git?rev=4fb0e0b70f868bccd529d45e1626de2f55a20b83#4fb0e0b70f868bccd529d45e1626de2f55a20b83"}]
//     evidence={"assembly":"\tcallq\t9594849","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393599840,"raw_hex":"e8e1679200","size":5,"soft_fail":false,"target_va":5403195312,"va":5393600458,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] intel_tex_2::bc7::compress_blocks_into
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"intel_tex_2","version":"0.4.0","source":"git+https://github.com/PounceLight/intel-tex-rs-2.git?rev=4fb0e0b70f868bccd529d45e1626de2f55a20b83#4fb0e0b70f868bccd529d45e1626de2f55a20b83"}]
//     evidence={"assembly":"\tcallq\t9594805","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393599840,"raw_hex":"e8b5679200","size":5,"soft_fail":false,"target_va":5403195440,"va":5393600630,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1727,"record_offset":53928,"parent_record_offset":53120,"ancestor_record_offset":53120,"inlinee":4759,"source_line":3186,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$30::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1727,"record_offset":55564,"parent_record_offset":53120,"ancestor_record_offset":53120,"inlinee":4954,"source_line":3825,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] asset_pipe::texture::BcMode::block_bytes
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":1727,"record_offset":53900,"parent_record_offset":53120,"ancestor_record_offset":53120,"inlinee":77766,"source_line":697,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] bytemuck::cast_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.16.0/src/lib.rs
//     evidence={"module":1727,"record_offset":55760,"parent_record_offset":53120,"ancestor_record_offset":53120,"inlinee":4645,"source_line":354,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1727,"record_offset":54288,"parent_record_offset":53120,"ancestor_record_offset":53120,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1727,"record_offset":53732,"parent_record_offset":53120,"ancestor_record_offset":53120,"inlinee":12536,"source_line":2758,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1727,"record_offset":54496,"parent_record_offset":53120,"ancestor_record_offset":53120,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1727,"record_offset":55872,"parent_record_offset":53120,"ancestor_record_offset":53120,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::chunks_exact
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1727,"record_offset":55340,"parent_record_offset":53120,"ancestor_record_offset":53120,"inlinee":30845,"source_line":1235,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] image::buffer_::ImageBuffer::pixels
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":1727,"record_offset":53280,"parent_record_offset":53120,"ancestor_record_offset":53120,"inlinee":17419,"source_line":720,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1727,"record_offset":54320,"parent_record_offset":53120,"ancestor_record_offset":53120,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1727,"record_offset":54204,"parent_record_offset":53120,"ancestor_record_offset":53120,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1727,"record_offset":54124,"parent_record_offset":54100,"ancestor_record_offset":53120,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1727,"record_offset":54888,"parent_record_offset":54864,"ancestor_record_offset":53120,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc_zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1727,"record_offset":54148,"parent_record_offset":54124,"ancestor_record_offset":53120,"inlinee":4772,"source_line":173,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1727,"record_offset":56056,"parent_record_offset":56032,"ancestor_record_offset":53120,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1727,"record_offset":54840,"parent_record_offset":54804,"ancestor_record_offset":53120,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate_zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1727,"record_offset":54100,"parent_record_offset":54060,"ancestor_record_offset":53120,"inlinee":4771,"source_line":259,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1727,"record_offset":56032,"parent_record_offset":55944,"ancestor_record_offset":53120,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1727,"record_offset":55660,"parent_record_offset":55636,"ancestor_record_offset":53120,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1727,"record_offset":54732,"parent_record_offset":54696,"ancestor_record_offset":53120,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_zeroed_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1727,"record_offset":53992,"parent_record_offset":53960,"ancestor_record_offset":53120,"inlinee":4769,"source_line":206,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1727,"record_offset":55972,"parent_record_offset":55944,"ancestor_record_offset":53120,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1727,"record_offset":55944,"parent_record_offset":55920,"ancestor_record_offset":53120,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1727,"record_offset":55708,"parent_record_offset":55684,"ancestor_record_offset":53120,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1727,"record_offset":55684,"parent_record_offset":55660,"ancestor_record_offset":53120,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1727,"record_offset":54060,"parent_record_offset":54024,"ancestor_record_offset":53120,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1727,"record_offset":54768,"parent_record_offset":54732,"ancestor_record_offset":53120,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_zeroed_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1727,"record_offset":54024,"parent_record_offset":53992,"ancestor_record_offset":53120,"inlinee":4770,"source_line":442,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1727,"record_offset":55920,"parent_record_offset":55896,"ancestor_record_offset":53120,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1727,"record_offset":55636,"parent_record_offset":55612,"ancestor_record_offset":53120,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1727,"record_offset":55612,"parent_record_offset":55588,"ancestor_record_offset":53120,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1727,"record_offset":54968,"parent_record_offset":54944,"ancestor_record_offset":53120,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1727,"record_offset":54660,"parent_record_offset":54616,"ancestor_record_offset":53120,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1727,"record_offset":54696,"parent_record_offset":54660,"ancestor_record_offset":53120,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1727,"record_offset":54536,"parent_record_offset":54496,"ancestor_record_offset":53120,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1727,"record_offset":55188,"parent_record_offset":55160,"ancestor_record_offset":53120,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1727,"record_offset":55588,"parent_record_offset":55564,"ancestor_record_offset":53120,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1727,"record_offset":55260,"parent_record_offset":55188,"ancestor_record_offset":53120,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1727,"record_offset":54944,"parent_record_offset":54616,"ancestor_record_offset":53120,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$3::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":1727,"record_offset":53960,"parent_record_offset":53928,"ancestor_record_offset":53120,"inlinee":5369,"source_line":53,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":1727,"record_offset":54576,"parent_record_offset":54536,"ancestor_record_offset":53120,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":1727,"record_offset":54616,"parent_record_offset":54576,"ancestor_record_offset":53120,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture::compress::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":1727,"record_offset":53860,"parent_record_offset":53836,"ancestor_record_offset":53120,"inlinee":77764,"source_line":710,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture::compress::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":1727,"record_offset":55128,"parent_record_offset":55104,"ancestor_record_offset":53120,"inlinee":77767,"source_line":725,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::internal::cast_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.16.0/src/internal.rs
//     evidence={"module":1727,"record_offset":55784,"parent_record_offset":55760,"ancestor_record_offset":53120,"inlinee":4647,"source_line":253,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::internal::try_cast_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.16.0/src/internal.rs
//     evidence={"module":1727,"record_offset":55808,"parent_record_offset":55784,"ancestor_record_offset":53120,"inlinee":4648,"source_line":353,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1727,"record_offset":55016,"parent_record_offset":54992,"ancestor_record_offset":53120,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1727,"record_offset":55104,"parent_record_offset":55040,"ancestor_record_offset":53120,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":1727,"record_offset":55524,"parent_record_offset":55496,"ancestor_record_offset":53120,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::any::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1727,"record_offset":53836,"parent_record_offset":53756,"ancestor_record_offset":53120,"inlinee":12540,"source_line":2765,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1727,"record_offset":55040,"parent_record_offset":55016,"ancestor_record_offset":53120,"inlinee":5298,"source_line":2539,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1727,"record_offset":54992,"parent_record_offset":54968,"ancestor_record_offset":53120,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1727,"record_offset":55160,"parent_record_offset":55104,"ancestor_record_offset":53120,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1727,"record_offset":53756,"parent_record_offset":53732,"ancestor_record_offset":53120,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1727,"record_offset":55832,"parent_record_offset":55808,"ancestor_record_offset":53120,"inlinee":8266,"source_line":338,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1727,"record_offset":53412,"parent_record_offset":53392,"ancestor_record_offset":53120,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1727,"record_offset":53436,"parent_record_offset":53412,"ancestor_record_offset":53120,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1727,"record_offset":56000,"parent_record_offset":55972,"ancestor_record_offset":53120,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1727,"record_offset":53368,"parent_record_offset":53344,"ancestor_record_offset":53120,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1727,"record_offset":53588,"parent_record_offset":53308,"ancestor_record_offset":53120,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1727,"record_offset":54376,"parent_record_offset":54344,"ancestor_record_offset":53120,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1727,"record_offset":54404,"parent_record_offset":54344,"ancestor_record_offset":53120,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1727,"record_offset":55428,"parent_record_offset":55400,"ancestor_record_offset":53120,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1727,"record_offset":55896,"parent_record_offset":55872,"ancestor_record_offset":53120,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1727,"record_offset":55228,"parent_record_offset":55188,"ancestor_record_offset":53120,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1727,"record_offset":55400,"parent_record_offset":55368,"ancestor_record_offset":53120,"inlinee":6336,"source_line":2001,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1727,"record_offset":53624,"parent_record_offset":53308,"ancestor_record_offset":53120,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1727,"record_offset":53680,"parent_record_offset":53652,"ancestor_record_offset":53120,"inlinee":4610,"source_line":431,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1727,"record_offset":53652,"parent_record_offset":53624,"ancestor_record_offset":53120,"inlinee":9466,"source_line":521,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::ChunksExact::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1727,"record_offset":55368,"parent_record_offset":55340,"ancestor_record_offset":53120,"inlinee":30847,"source_line":1866,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$90::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1727,"record_offset":53804,"parent_record_offset":53780,"ancestor_record_offset":53120,"inlinee":17422,"source_line":1911,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1727,"record_offset":54228,"parent_record_offset":54204,"ancestor_record_offset":53120,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1727,"record_offset":54252,"parent_record_offset":54228,"ancestor_record_offset":53120,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::image_buffer_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":1727,"record_offset":53344,"parent_record_offset":53308,"ancestor_record_offset":53120,"inlinee":7815,"source_line":807,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::inner_pixels
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":1727,"record_offset":53308,"parent_record_offset":53280,"ancestor_record_offset":53120,"inlinee":9503,"source_line":713,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":1727,"record_offset":53780,"parent_record_offset":53756,"ancestor_record_offset":53120,"inlinee":17421,"source_line":34,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$34::image_buffer_len::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":1727,"record_offset":53392,"parent_record_offset":53368,"ancestor_record_offset":53120,"inlinee":7817,"source_line":809,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$34::image_buffer_len::closure$1
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":1727,"record_offset":53500,"parent_record_offset":53476,"ancestor_record_offset":53120,"inlinee":7818,"source_line":810,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1727,"record_offset":54432,"parent_record_offset":54344,"ancestor_record_offset":53120,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1727,"record_offset":54344,"parent_record_offset":54320,"ancestor_record_offset":53120,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1727,"record_offset":54456,"parent_record_offset":54432,"ancestor_record_offset":53120,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::compress::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 710
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12FC4
// 首条位置: {"module":1727,"inlinee":77764}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::compress::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 725
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12FC7
// 首条位置: {"module":1727,"inlinee":77767}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::compress::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 741
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A678
// 首条位置: {"module":3168,"inlinee":108152}
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
// 完整结构及原始类型记录: texture.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset_pipe::texture::BcMode
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 0
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset_pipe::texture::TextureAssetLibraryLoadResult
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>", "constructor": "enum2$", "arguments": ["asset_pipe::texture::TextureAssetLibraryLoadResult"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::once_lock::impl$0::get_or_init::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,country_core::startup::load_textures::load_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures> >", "constructor": "std::sync::once_lock::impl$0::get_or_init::closure_env$0", "arguments": ["enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>", "country_core::startup::load_textures::load_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::once_lock::impl$0::initialize::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,std::sync::once_lock::impl$0::get_or_init::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,country_core::startup::load_textures::load_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures> >,never$>", "constructor": "std::sync::once_lock::impl$0::initialize::closure_env$0", "arguments": ["enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>", "std::sync::once_lock::impl$0::get_or_init::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,country_core::startup::load_textures::load_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures> >", "never$"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::poison::once::impl$2::call_once_force::closure$0<std::sync::once_lock::impl$0::initialize::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,std::sync::once_lock::impl$0::get_or_init::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,country_core::startup::load_textures::load_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures> >,never$> >", "constructor": "std::sync::poison::once::impl$2::call_once_force::closure$0", "arguments": ["std::sync::once_lock::impl$0::initialize::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,std::sync::once_lock::impl$0::get_or_init::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,country_core::startup::load_textures::load_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures> >,never$>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::once_lock::impl$0::get_or_init::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,country_core::startup::load_textures::load_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures> >", "constructor": "std::sync::once_lock::impl$0::get_or_init::closure_env$0", "arguments": ["enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>", "country_core::startup::load_textures::load_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::once_lock::OnceLock::initialize<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,std::sync::once_lock::impl$0::get_or_init::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,country_core::startup::load_textures::load_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures> >,never$>", "constructor": "std::sync::once_lock::OnceLock::initialize", "arguments": ["enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>", "std::sync::once_lock::impl$0::get_or_init::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,country_core::startup::load_textures::load_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures> >", "never$"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::once_lock::OnceLock::initialize<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,std::sync::once_lock::impl$0::get_or_init::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,country_core::startup::load_textures::load_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures> >,never$>", "constructor": "std::sync::once_lock::OnceLock::initialize", "arguments": ["enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>", "std::sync::once_lock::impl$0::get_or_init::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,country_core::startup::load_textures::load_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures> >", "never$"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::once_lock::impl$0::initialize::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,std::sync::once_lock::impl$0::get_or_init::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,country_core::startup::load_textures::load_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures> >,never$>", "constructor": "std::sync::once_lock::impl$0::initialize::closure_env$0", "arguments": ["enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>", "std::sync::once_lock::impl$0::get_or_init::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,country_core::startup::load_textures::load_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures> >", "never$"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::poison::once::impl$2::call_once_force::closure_env$0<std::sync::once_lock::impl$0::initialize::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,std::sync::once_lock::impl$0::get_or_init::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,country_core::startup::load_textures::load_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures> >,never$> >", "constructor": "std::sync::poison::once::impl$2::call_once_force::closure_env$0", "arguments": ["std::sync::once_lock::impl$0::initialize::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,std::sync::once_lock::impl$0::get_or_init::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,country_core::startup::load_textures::load_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures> >,never$>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<std::sync::poison::once::impl$2::call_once_force::closure_env$0<std::sync::once_lock::impl$0::initialize::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,std::sync::once_lock::impl$0::get_or_init::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,country_core::startup::load_textures::load_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures> >,never$> >,tuple$<ref$<std::sync::poison::once::OnceState> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["std::sync::poison::once::impl$2::call_once_force::closure_env$0<std::sync::once_lock::impl$0::initialize::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,std::sync::once_lock::impl$0::get_or_init::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,country_core::startup::load_textures::load_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures> >,never$> >", "tuple$<ref$<std::sync::poison::once::OnceState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset_pipe::texture::TextureAssetLibraryMeta
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<asset_pipe::texture::TextureAssetLibraryMeta,anyhow::Error>", "constructor": "core::result::Result", "arguments": ["asset_pipe::texture::TextureAssetLibraryMeta", "anyhow::Error"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<asset_pipe::texture::TextureAssetLibraryMeta,anyhow::Error> >", "constructor": "enum2$", "arguments": ["core::result::Result<asset_pipe::texture::TextureAssetLibraryMeta,anyhow::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<asset_pipe::texture::TextureAssetLibraryMeta,anyhow::Error> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<asset_pipe::texture::TextureAssetLibraryMeta,anyhow::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::marker::PhantomData<asset_pipe::texture::TextureAssetLibraryMeta>", "constructor": "core::marker::PhantomData", "arguments": ["asset_pipe::texture::TextureAssetLibraryMeta"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::options::Options::from_str_seed<core::marker::PhantomData<asset_pipe::texture::TextureAssetLibraryMeta>,asset_pipe::texture::TextureAssetLibraryMeta>", "constructor": "ron::options::Options::from_str_seed", "arguments": ["core::marker::PhantomData<asset_pipe::texture::TextureAssetLibraryMeta>", "asset_pipe::texture::TextureAssetLibraryMeta"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::options::Options::to_string<asset_pipe::texture::TextureAssetLibraryMeta>", "constructor": "ron::options::Options::to_string", "arguments": ["asset_pipe::texture::TextureAssetLibraryMeta"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset_pipe::texture::TextureAssetMeta
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 60
// [PDB 泛型关系，非字段关系] {"type": "tuple$<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta>", "constructor": "tuple$", "arguments": ["relative_path::RelativePathBuf", "asset_pipe::texture::TextureAssetMeta"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta>,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error>", "constructor": "core::result::Result", "arguments": ["asset_pipe::texture::TextureAssetMeta", "anyhow::Error"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> >", "constructor": "enum2$", "arguments": ["core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >", "constructor": "tuple$", "arguments": ["relative_path::RelativePathBuf", "enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "easy_parallel::Parallel<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "easy_parallel::Parallel", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<easy_parallel::Parallel<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["easy_parallel::Parallel<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpsc::Sender<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "std::sync::mpsc::Sender", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpsc::Sender<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpsc::Sender<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpsc::Receiver<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "std::sync::mpsc::Receiver", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpsc::Receiver<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpsc::Receiver<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpsc::SendError<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "std::sync::mpsc::SendError", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpsc::SendError<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpsc::SendError<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Packet<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "std::sync::mpmc::zero::Packet", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpmc::zero::Packet<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpmc::zero::Packet<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<std::sync::mpsc::Receiver<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["std::sync::mpsc::Receiver<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<std::sync::mpsc::Receiver<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<std::sync::mpsc::Receiver<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<std::sync::mpsc::Receiver<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["std::sync::mpsc::Receiver<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<std::sync::mpsc::Receiver<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<std::sync::mpsc::Receiver<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "std::sync::mpmc::list::Channel", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Counter<std::sync::mpmc::list::Channel<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >", "constructor": "std::sync::mpmc::counter::Counter", "arguments": ["std::sync::mpmc::list::Channel<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpmc::counter::Counter<std::sync::mpmc::list::Channel<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpmc::counter::Counter<std::sync::mpmc::list::Channel<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "std::sync::mpmc::array::Channel", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Counter<std::sync::mpmc::array::Channel<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >", "constructor": "std::sync::mpmc::counter::Counter", "arguments": ["std::sync::mpmc::array::Channel<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpmc::counter::Counter<std::sync::mpmc::array::Channel<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpmc::counter::Counter<std::sync::mpmc::array::Channel<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "assoc$<Output,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "assoc$", "arguments": ["Output", "tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce<tuple$<>,assoc$<Output,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >", "constructor": "core::ops::function::FnOnce", "arguments": ["tuple$<>", "assoc$<Output,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >,core::marker::Send>", "constructor": "dyn$", "arguments": ["core::ops::function::FnOnce<tuple$<>,assoc$<Output,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >", "core::marker::Send"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >,core::marker::Send>,alloc::alloc::Global>", "constructor": "alloc::boxed::Box", "arguments": ["dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >,core::marker::Send>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >,core::marker::Send>,alloc::alloc::Global>,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >,core::marker::Send>,alloc::alloc::Global>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >,core::marker::Send>,alloc::alloc::Global>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >,core::marker::Send>,alloc::alloc::Global>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::Keys<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta>", "constructor": "std::collections::hash::map::Keys", "arguments": ["relative_path::RelativePathBuf", "asset_pipe::texture::TextureAssetMeta"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<ref$<relative_path::RelativePathBuf>,std::collections::hash::map::Keys<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["ref$<relative_path::RelativePathBuf>", "std::collections::hash::map::Keys<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "easy_parallel::Parallel::run<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "easy_parallel::Parallel::run", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<asset_pipe::texture::TextureAssetMeta>", "constructor": "core::option::Option", "arguments": ["asset_pipe::texture::TextureAssetMeta"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<asset_pipe::texture::TextureAssetMeta> >", "constructor": "enum2$", "arguments": ["core::option::Option<asset_pipe::texture::TextureAssetMeta>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<enum2$<core::option::Option<asset_pipe::texture::TextureAssetMeta> >,relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>", "constructor": "tuple$", "arguments": ["enum2$<core::option::Option<asset_pipe::texture::TextureAssetMeta> >", "relative_path::RelativePathBuf", "asset::texture::desc::TextureAssetDesc"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "easy_parallel::Parallel::each<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >,tuple$<enum2$<core::option::Option<asset_pipe::texture::TextureAssetMeta> >,relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,core::iter::adapters::map::Map<std::collections::hash::map::IntoIter<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,asset_pipe::texture::compile_texture_assets::closure_env$3>,asset_pipe::texture::compile_texture_assets::closure_env$4>", "constructor": "easy_parallel::Parallel::each", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >", "tuple$<enum2$<core::option::Option<asset_pipe::texture::TextureAssetMeta> >,relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>", "core::iter::adapters::map::Map<std::collections::hash::map::IntoIter<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,asset_pipe::texture::compile_texture_assets::closure_env$3>", "asset_pipe::texture::compile_texture_assets::closure_env$4"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "easy_parallel::impl$0::each::closure_env$0<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >,tuple$<enum2$<core::option::Option<asset_pipe::texture::TextureAssetMeta> >,relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,core::iter::adapters::map::Map<std::collections::hash::map::IntoIter<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,asset_pipe::texture::compile_texture_assets::closure_env$3>,asset_pipe::texture::compile_texture_assets::closure_env$4>", "constructor": "easy_parallel::impl$0::each::closure_env$0", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >", "tuple$<enum2$<core::option::Option<asset_pipe::texture::TextureAssetMeta> >,relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>", "core::iter::adapters::map::Map<std::collections::hash::map::IntoIter<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,asset_pipe::texture::compile_texture_assets::closure_env$3>", "asset_pipe::texture::compile_texture_assets::closure_env$4"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<easy_parallel::impl$0::each::closure_env$0<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >,tuple$<enum2$<core::option::Option<asset_pipe::texture::TextureAssetMeta> >,relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,core::iter::adapters::map::Map<std::collections::hash::map::IntoIter<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,asset_pipe::texture::compile_texture_assets::closure_env$3>,asset_pipe::texture::compile_texture_assets::closure_env$4>,tuple$<> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["easy_parallel::impl$0::each::closure_env$0<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >,tuple$<enum2$<core::option::Option<asset_pipe::texture::TextureAssetMeta> >,relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,core::iter::adapters::map::Map<std::collections::hash::map::IntoIter<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,asset_pipe::texture::compile_texture_assets::closure_env$3>,asset_pipe::texture::compile_texture_assets::closure_env$4>", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "easy_parallel::impl$0::finish::closure_env$0<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >,alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >,core::marker::Send>,alloc::alloc::Global>,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "easy_parallel::impl$0::finish::closure_env$0", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >", "alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >,core::marker::Send>,alloc::alloc::Global>", "tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<easy_parallel::impl$0::finish::closure_env$0<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >,alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >,core::marker::Send>,alloc::alloc::Global>,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >,tuple$<> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["easy_parallel::impl$0::finish::closure_env$0<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >,alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >,core::marker::Send>,alloc::alloc::Global>,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >,core::marker::Send>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >,core::marker::Send>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<easy_parallel::impl$0::finish::closure_env$0<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >,alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >,core::marker::Send>,alloc::alloc::Global>,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["easy_parallel::impl$0::finish::closure_env$0<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >,alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >,core::marker::Send>,alloc::alloc::Global>,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<easy_parallel::impl$0::each::closure_env$0<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >,tuple$<enum2$<core::option::Option<asset_pipe::texture::TextureAssetMeta> >,relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,core::iter::adapters::map::Map<std::collections::hash::map::IntoIter<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,asset_pipe::texture::compile_texture_assets::closure_env$3>,asset_pipe::texture::compile_texture_assets::closure_env$4> >", "constructor": "core::ptr::drop_in_place", "arguments": ["easy_parallel::impl$0::each::closure_env$0<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >,tuple$<enum2$<core::option::Option<asset_pipe::texture::TextureAssetMeta> >,relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,core::iter::adapters::map::Map<std::collections::hash::map::IntoIter<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,asset_pipe::texture::compile_texture_assets::closure_env$3>,asset_pipe::texture::compile_texture_assets::closure_env$4>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["relative_path::RelativePathBuf", "asset_pipe::texture::TextureAssetMeta", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["relative_path::RelativePathBuf", "asset_pipe::texture::TextureAssetMeta", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1", "arguments": ["tuple$<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta,std::hash::random::RandomState> >,tuple$<ptr_mut$<u8> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta,std::hash::random::RandomState> >", "tuple$<ptr_mut$<u8> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::equivalent_key::closure_env$0<relative_path::RelativePathBuf,relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta>", "constructor": "hashbrown::map::equivalent_key::closure_env$0", "arguments": ["relative_path::RelativePathBuf", "relative_path::RelativePathBuf", "asset_pipe::texture::TextureAssetMeta"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::remove_entry<tuple$<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env$0<relative_path::RelativePathBuf,relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta> >", "constructor": "hashbrown::raw::RawTable::remove_entry", "arguments": ["tuple$<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta>", "alloc::alloc::Global", "hashbrown::map::equivalent_key::closure_env$0<relative_path::RelativePathBuf,relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<std::sync::mpsc::Receiver<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["std::sync::mpsc::Receiver<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::disconnect_senders<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "std::sync::mpmc::list::Channel::disconnect_senders", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::disconnect_receivers<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "std::sync::mpmc::list::Channel::disconnect_receivers", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::recv<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "std::sync::mpmc::list::Channel::recv", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::impl$3::recv::closure$1<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "std::sync::mpmc::list::impl$3::recv::closure$1", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::send<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "std::sync::mpmc::list::Channel::send", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "std::sync::mpmc::zero::Channel", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::impl$6::drop::closure_env$2<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "std::sync::mpmc::impl$6::drop::closure_env$2", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Sender::release<std::sync::mpmc::zero::Channel<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >,std::sync::mpmc::impl$6::drop::closure_env$2<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >", "constructor": "std::sync::mpmc::counter::Sender::release", "arguments": ["std::sync::mpmc::zero::Channel<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "std::sync::mpmc::impl$6::drop::closure_env$2<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::impl$20::drop::closure_env$2<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "std::sync::mpmc::impl$20::drop::closure_env$2", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Receiver::release<std::sync::mpmc::zero::Channel<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >,std::sync::mpmc::impl$20::drop::closure_env$2<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >", "constructor": "std::sync::mpmc::counter::Receiver::release", "arguments": ["std::sync::mpmc::zero::Channel<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "std::sync::mpmc::impl$20::drop::closure_env$2<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel::disconnect_receivers<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "std::sync::mpmc::array::Channel::disconnect_receivers", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel::recv<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "std::sync::mpmc::array::Channel::recv", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::impl$1::recv::closure$1<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "std::sync::mpmc::array::impl$1::recv::closure$1", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel::send<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "std::sync::mpmc::array::Channel::send", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::impl$1::send::closure$0<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "std::sync::mpmc::array::impl$1::send::closure$0", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$204::deserialize::MapVisitor<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta,std::hash::random::RandomState>", "constructor": "serde::de::impls::impl$204::deserialize::MapVisitor", "arguments": ["relative_path::RelativePathBuf", "asset_pipe::texture::TextureAssetMeta", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$2::deserialize_map::impl$0::fmt<serde::de::impls::impl$204::deserialize::MapVisitor<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta,std::hash::random::RandomState> >", "constructor": "ron::de::impl$2::deserialize_map::impl$0::fmt", "arguments": ["serde::de::impls::impl$204::deserialize::MapVisitor<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$2::deserialize_map<serde::de::impls::impl$204::deserialize::MapVisitor<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta,std::hash::random::RandomState> >", "constructor": "ron::de::impl$2::deserialize_map", "arguments": ["serde::de::impls::impl$204::deserialize::MapVisitor<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::HashMap<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta,std::hash::random::RandomState>", "constructor": "std::collections::hash::map::HashMap", "arguments": ["relative_path::RelativePathBuf", "asset_pipe::texture::TextureAssetMeta", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,std::collections::hash::map::HashMap<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta,std::hash::random::RandomState> >", "constructor": "ron::ser::impl$11::serialize_field", "arguments": ["ref_mut$<alloc::string::String>", "std::collections::hash::map::HashMap<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel::disconnect<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "std::sync::mpmc::zero::Channel::disconnect", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel::recv<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "std::sync::mpmc::zero::Channel::recv", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::impl$3::recv::closure$1<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "std::sync::mpmc::zero::impl$3::recv::closure$1", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel::send<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "std::sync::mpmc::zero::Channel::send", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::impl$3::send::closure$0<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "std::sync::mpmc::zero::impl$3::send::closure$0", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<tuple$<>,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "core::result::Result", "arguments": ["tuple$<>", "tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<tuple$<>,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >", "constructor": "enum2$", "arguments": ["core::result::Result<tuple$<>,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<tuple$<>,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::error::SendTimeoutError<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "std::sync::mpmc::error::SendTimeoutError", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<std::sync::mpmc::error::SendTimeoutError<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >", "constructor": "enum2$", "arguments": ["std::sync::mpmc::error::SendTimeoutError<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<tuple$<>,enum2$<std::sync::mpmc::error::SendTimeoutError<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > > >", "constructor": "core::result::Result", "arguments": ["tuple$<>", "enum2$<std::sync::mpmc::error::SendTimeoutError<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<tuple$<>,enum2$<std::sync::mpmc::error::SendTimeoutError<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > > > >", "constructor": "enum2$", "arguments": ["core::result::Result<tuple$<>,enum2$<std::sync::mpmc::error::SendTimeoutError<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,enum2$<std::sync::mpmc::error::SendTimeoutError<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<tuple$<>,enum2$<std::sync::mpmc::error::SendTimeoutError<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::impl$3::recv::closure_env$1<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "std::sync::mpmc::zero::impl$3::recv::closure_env$1", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<std::sync::mpmc::zero::impl$3::recv::closure_env$1<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >", "constructor": "core::option::Option", "arguments": ["std::sync::mpmc::zero::impl$3::recv::closure_env$1<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::recv::closure_env$1<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > > >", "constructor": "enum2$", "arguments": ["core::option::Option<std::sync::mpmc::zero::impl$3::recv::closure_env$1<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::recv::closure_env$1<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::recv::closure_env$1<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::impl$3::send::closure_env$0<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "std::sync::mpmc::zero::impl$3::send::closure_env$0", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >", "constructor": "core::option::Option", "arguments": ["std::sync::mpmc::zero::impl$3::send::closure_env$0<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > > >", "constructor": "enum2$", "arguments": ["core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::Sender::send<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "std::sync::mpmc::Sender::send", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<std::sync::mpmc::error::SendTimeoutError<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<std::sync::mpmc::error::SendTimeoutError<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpsc::impl$18::fmt<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "std::sync::mpsc::impl$18::fmt", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: texture.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: texture.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: asset_pipe::texture::BcMode::block_bytes
// 信息 ID: cabe3bf5fdf92850a65dc13e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture::compress; module=1727, inline_record=53900; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::_::impl$0::deserialize
// 信息 ID: f321b93f31ecb22c932d82e0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::options::Options::from_str_seed<core::marker::PhantomData<asset_pipe::texture::TextureAssetLibraryMeta>,asset_pipe::texture::TextureAssetLibraryMeta>; module=3166, inline_record=564; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_map<serde::de::impls::impl$204::deserialize::MapVisitor<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta,std::hash::random::RandomState> >; module=4139, inline_record=17424; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: 05e001175d8e324c9582e84e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset_pipe::texture::_::impl$0::deserialize::__Visitor>; module=4139, inline_record=32556; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset_pipe::texture::_::impl$0::deserialize::__Visitor>; module=4139, inline_record=36192; 不是独立调用边。
// [PDB 内联宿主] ron::de::id::impl$1::deserialize_identifier<asset_pipe::texture::_::impl$0::deserialize::__FieldVisitor>; module=4150, inline_record=4260; 不是独立调用边。
// [PDB 内联宿主] ron::de::id::impl$1::deserialize_identifier<asset_pipe::texture::_::impl$0::deserialize::__FieldVisitor>; module=4150, inline_record=4536; 不是独立调用边。
// [PDB 内联宿主] ron::de::id::impl$1::deserialize_identifier<asset_pipe::texture::_::impl$0::deserialize::__FieldVisitor>; module=4150, inline_record=5188; 不是独立调用边。
// [PDB 内联宿主] ron::de::id::impl$1::deserialize_identifier<asset_pipe::texture::_::impl$0::deserialize::__FieldVisitor>; module=4150, inline_record=5612; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: 5b38d72ef6f9fef120d3a7ac
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset_pipe::texture::_::impl$0::deserialize::__Visitor>; module=4139, inline_record=32136; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset_pipe::texture::_::impl$0::deserialize::__Visitor>; module=4139, inline_record=32292; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset_pipe::texture::_::impl$0::deserialize::__Visitor>; module=4139, inline_record=32416; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset_pipe::texture::_::impl$0::deserialize::__Visitor>; module=4139, inline_record=35792; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset_pipe::texture::_::impl$0::deserialize::__Visitor>; module=4139, inline_record=35948; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset_pipe::texture::_::impl$0::deserialize::__Visitor>; module=4139, inline_record=36068; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: 67d7e64e1513a04361c2ca93
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset_pipe::texture::_::impl$0::deserialize::__Visitor>; module=4139, inline_record=31756; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset_pipe::texture::_::impl$0::deserialize::__Visitor>; module=4139, inline_record=35432; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::_::impl$0::serialize<ref_mut$<ron::ser::Serializer<ref_mut$<alloc::string::String> > > >
// 信息 ID: 9194ef4fce39ed5b08fad747
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref_mut$ -> ["alloc::string::String"]; 字段布局 = unknown。
// [类型关系] ron::ser::Serializer -> ["ref_mut$<alloc::string::String>"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["ron::ser::Serializer<ref_mut$<alloc::string::String> >"]; 字段布局 = unknown。
// [类型关系] asset_pipe::texture::_::impl$0::serialize -> ["ref_mut$<ron::ser::Serializer<ref_mut$<alloc::string::String> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::options::Options::to_string<asset_pipe::texture::TextureAssetLibraryMeta> @ 0x20d87e2 -> 0x20e33e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,std::collections::hash::map::HashMap<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta,std::hash::random::RandomState> > @ 0x24526c4 -> 0x20e3560 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::apply_transfer_function
// 信息 ID: 13f103afbc9b4663a566dd0d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture::process_regular_image; module=1727, inline_record=49164; 不是独立调用边。
// [PDB 内联宿主] asset_pipe::texture::with_rgba8<asset_pipe::texture::process_regular_image::closure_env$4,tuple$<> >; module=3187, inline_record=1408; 不是独立调用边。
// [PDB 内联宿主] asset_pipe::texture::with_rgba8<asset_pipe::texture::process_regular_image::closure_env$2,alloc::vec::Vec<u8,alloc::alloc::Global> >; module=3187, inline_record=5988; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::compile_or_load_texture_assets
// 信息 ID: 17f6fb64b024c9bc0b4d8158
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] std::sync::poison::once::impl$2::call_once_force::closure$0<std::sync::once_lock::impl$0::initialize::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,std::sync::once_lock::impl$0::get_or_init::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,country_core::startup::load_textures::load_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures> >,never$> > | core::ops::function::FnOnce::call_once<std::sync::poison::once::impl$2::call_once_force::closure_env$0<std::sync::once_lock::impl$0::initialize::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,std::sync::once_lock::impl$0::get_or_init::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,country_core::startup::load_textures::load_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures> >,never$> >,tuple$<ref$<std::sync::poison::once::OnceState> > > @ 0xb65163 -> 0x17b7300 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::compile_or_load_texture_assets::closure$0
// 信息 ID: e0af5f8cef3c05c6a5b8173d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "asset_pipe::texture::compile_or_load_texture_assets", "captures": "unknown"}
// [闭包环境类型引用] asset_pipe::texture::compile_or_load_texture_assets::closure_env$0 <- anyhow::context::impl$0::with_context<alloc::string::String,std::io::error::Error,alloc::string::String,asset_pipe::texture::compile_or_load_texture_assets::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<alloc::string::String,std::io::error::Error,alloc::string::String,asset_pipe::texture::compile_or_load_texture_assets::closure_env$0>; module=3165, inline_record=2784; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::compile_or_load_texture_assets::closure$1
// 信息 ID: 4fcf73fecf2501f17cd0246d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "asset_pipe::texture::compile_or_load_texture_assets", "captures": "unknown"}
// [闭包环境类型引用] asset_pipe::texture::compile_or_load_texture_assets::closure_env$1 <- anyhow::context::impl$0::with_context<alloc::vec::Vec<relative_path::RelativePathBuf,alloc::alloc::Global>,ron::error::SpannedError,alloc::string::String,asset_pipe::texture::compile_or_load_texture_assets::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<alloc::vec::Vec<relative_path::RelativePathBuf,alloc::alloc::Global>,ron::error::SpannedError,alloc::string::String,asset_pipe::texture::compile_or_load_texture_assets::closure_env$1>; module=3165, inline_record=2116; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::compile_or_load_texture_assets::closure$2
// 信息 ID: f1af23b332690c57cb8a1d6e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "asset_pipe::texture::compile_or_load_texture_assets", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture::compile_or_load_texture_assets; module=1727, inline_record=12552; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::compile_texture_asset
// 信息 ID: 125ea9d8bc07356be0298914
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<easy_parallel::impl$0::each::closure_env$0<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >,tuple$<enum2$<core::option::Option<asset_pipe::texture::TextureAssetMeta> >,relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,core::iter::adapters::map::Map<std::collections::hash::map::IntoIter<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,asset_pipe::texture::compile_texture_assets::closure_env$3>,asset_pipe::texture::compile_texture_assets::closure_env$4>,tuple$<> > @ 0x20ddbee -> 0x17b8f50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::compile_texture_asset::closure$0
// 信息 ID: a92cf0764a801e79cc893873
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "asset_pipe::texture::compile_texture_asset", "captures": "unknown"}
// [闭包环境类型引用] asset_pipe::texture::compile_texture_asset::closure_env$0 <- anyhow::context::impl$0::with_context<std::fs::File,std::io::error::Error,alloc::string::String,asset_pipe::texture::compile_texture_asset::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<std::fs::File,std::io::error::Error,alloc::string::String,asset_pipe::texture::compile_texture_asset::closure_env$0>; module=3165, inline_record=5464; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::compile_texture_assets
// 信息 ID: ff5a1163015975ecf0fc8108
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::backend::texture_watch::texture_watch @ 0xaab4f2 -> 0x17b7790 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset_pipe::texture::compile_or_load_texture_assets @ 0x17b754e -> 0x17b7790 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::compile_texture_assets::closure$1
// 信息 ID: ac8d987748f936099d6bbfe3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "asset_pipe::texture::compile_texture_assets", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture::compile_texture_assets; module=1727, inline_record=16252; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::compile_texture_assets::closure$2
// 信息 ID: 66fe06ad3134865e912edfa6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "asset_pipe::texture::compile_texture_assets", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture::compile_texture_assets; module=1727, inline_record=16936; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::compile_texture_assets::closure$3
// 信息 ID: b0ba657ebe48ca7abb9d72ed
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "asset_pipe::texture::compile_texture_assets", "captures": "unknown"}
// [闭包环境类型引用] asset_pipe::texture::compile_texture_assets::closure_env$3 <- core::ptr::drop_in_place<core::iter::adapters::map::Map<std::collections::hash::map::IntoIter<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,asset_pipe::texture::compile_texture_assets::closure_env$3> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] asset_pipe::texture::compile_texture_assets::closure_env$3 <- easy_parallel::Parallel::each<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >,tuple$<enum2$<core::option::Option<asset_pipe::texture::TextureAssetMeta> >,relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,core::iter::adapters::map::Map<std::collections::hash::map::IntoIter<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,asset_pipe::texture::compile_texture_assets::closure_env$3>,asset_pipe::texture::compile_texture_assets::closure_env$4>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] asset_pipe::texture::compile_texture_assets::closure_env$3 <- core::ops::function::FnOnce::call_once<easy_parallel::impl$0::each::closure_env$0<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >,tuple$<enum2$<core::option::Option<asset_pipe::texture::TextureAssetMeta> >,relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,core::iter::adapters::map::Map<std::collections::hash::map::IntoIter<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,asset_pipe::texture::compile_texture_assets::closure_env$3>,asset_pipe::texture::compile_texture_assets::closure_env$4>,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] asset_pipe::texture::compile_texture_assets::closure_env$3 <- core::ptr::drop_in_place<easy_parallel::impl$0::each::closure_env$0<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >,tuple$<enum2$<core::option::Option<asset_pipe::texture::TextureAssetMeta> >,relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,core::iter::adapters::map::Map<std::collections::hash::map::IntoIter<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,asset_pipe::texture::compile_texture_assets::closure_env$3>,asset_pipe::texture::compile_texture_assets::closure_env$4> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] asset_pipe::texture::compile_texture_assets::closure_env$3 <- core::ops::function::impls::impl$4::call_once<tuple$<tuple$<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc> >,asset_pipe::texture::compile_texture_assets::closure_env$3>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 5 条。
// [PDB 内联宿主] core::ops::function::impls::impl$4::call_once<tuple$<tuple$<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc> >,asset_pipe::texture::compile_texture_assets::closure_env$3>; module=3187, inline_record=532; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::compile_texture_assets::closure$4
// 信息 ID: 7c5ae74a8cc444ea55c499a5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "asset_pipe::texture::compile_texture_assets", "captures": "unknown"}
// [闭包环境类型引用] asset_pipe::texture::compile_texture_assets::closure_env$4 <- easy_parallel::Parallel::each<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >,tuple$<enum2$<core::option::Option<asset_pipe::texture::TextureAssetMeta> >,relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,core::iter::adapters::map::Map<std::collections::hash::map::IntoIter<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,asset_pipe::texture::compile_texture_assets::closure_env$3>,asset_pipe::texture::compile_texture_assets::closure_env$4>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] asset_pipe::texture::compile_texture_assets::closure_env$4 <- core::ops::function::FnOnce::call_once<easy_parallel::impl$0::each::closure_env$0<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >,tuple$<enum2$<core::option::Option<asset_pipe::texture::TextureAssetMeta> >,relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,core::iter::adapters::map::Map<std::collections::hash::map::IntoIter<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,asset_pipe::texture::compile_texture_assets::closure_env$3>,asset_pipe::texture::compile_texture_assets::closure_env$4>,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] asset_pipe::texture::compile_texture_assets::closure_env$4 <- core::ptr::drop_in_place<easy_parallel::impl$0::each::closure_env$0<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >,tuple$<enum2$<core::option::Option<asset_pipe::texture::TextureAssetMeta> >,relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,core::iter::adapters::map::Map<std::collections::hash::map::IntoIter<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,asset_pipe::texture::compile_texture_assets::closure_env$3>,asset_pipe::texture::compile_texture_assets::closure_env$4> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<easy_parallel::impl$0::each::closure_env$0<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >,tuple$<enum2$<core::option::Option<asset_pipe::texture::TextureAssetMeta> >,relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,core::iter::adapters::map::Map<std::collections::hash::map::IntoIter<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,asset_pipe::texture::compile_texture_assets::closure_env$3>,asset_pipe::texture::compile_texture_assets::closure_env$4>,tuple$<> >; module=3169, inline_record=8992; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::compress
// 信息 ID: e37093c4c35cf8756dcd85c8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset_pipe::texture::with_rgba8<asset_pipe::texture::process_regular_image::closure_env$4,tuple$<> > @ 0x20e2cdc -> 0x17bcd60 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset_pipe::texture::with_rgba8<asset_pipe::texture::process_regular_image::closure_env$4,tuple$<> > @ 0x20e2d6b -> 0x17bcd60 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset_pipe::texture::with_rgba8<asset_pipe::texture::process_regular_image::closure_env$2,alloc::vec::Vec<u8,alloc::alloc::Global> > @ 0x20e318d -> 0x17bcd60 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset_pipe::texture::with_rgba8<asset_pipe::texture::process_regular_image::closure_env$2,alloc::vec::Vec<u8,alloc::alloc::Global> > @ 0x20e31d9 -> 0x17bcd60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::compress::closure$0
// 信息 ID: f8dd0e9d3314ad7285d010ad
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "asset_pipe::texture::compress", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture::compress; module=1727, inline_record=53860; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::compress::closure$1
// 信息 ID: 57bb98661f6f6fca8f8e0273
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "asset_pipe::texture::compress", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture::compress; module=1727, inline_record=55128; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::compress::closure$2
// 信息 ID: d206c20a5470e0d6d8a3ee95
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "asset_pipe::texture::compress", "captures": "unknown"}
// [闭包环境类型引用] asset_pipe::texture::compress::closure_env$2 <- alloc::vec::spec_from_iter::impl$0::from_iter<array$<u8,2>,core::iter::adapters::map::Map<core::slice::iter::ChunksExact<u8>,asset_pipe::texture::compress::closure_env$2> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<array$<u8,2>,core::iter::adapters::map::Map<core::slice::iter::ChunksExact<u8>,asset_pipe::texture::compress::closure_env$2> >; module=3168, inline_record=4792; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::impl$1::default
// 信息 ID: 36839188d86fbd529e855c2c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture::compile_texture_assets; module=1727, inline_record=15112; 不是独立调用边。
// [PDB 内联宿主] asset_pipe::texture::compile_texture_assets; module=1727, inline_record=17268; 不是独立调用边。
// [PDB 内联宿主] asset_pipe::texture::compile_texture_assets; module=1727, inline_record=19668; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::impl$2::fmt
// 信息 ID: c2f3ed0423d22e5df766966b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::pad_image
// 信息 ID: 0aed06f0a556ddcbebcbad12
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture::process_regular_image::closure$3; module=1727, inline_record=52068; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::process_dds_image
// 信息 ID: fcafda5ef7f50156fd183c4d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture::compile_texture_asset; module=1727, inline_record=40428; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::process_dds_image::closure$0
// 信息 ID: 15782b53821db685dd9913c9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "asset_pipe::texture::process_dds_image", "captures": "unknown"}
// [闭包环境类型引用] asset_pipe::texture::process_dds_image::closure_env$0 <- anyhow::context::impl$0::with_context<alloc::vec::Vec<u8,alloc::alloc::Global>,std::io::error::Error,alloc::string::String,asset_pipe::texture::process_dds_image::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<alloc::vec::Vec<u8,alloc::alloc::Global>,std::io::error::Error,alloc::string::String,asset_pipe::texture::process_dds_image::closure_env$0>; module=3165, inline_record=4808; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::process_dds_image::closure$1
// 信息 ID: 072cb9224df6be2ec01aaeed
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "asset_pipe::texture::process_dds_image", "captures": "unknown"}
// [闭包环境类型引用] asset_pipe::texture::process_dds_image::closure_env$1 <- anyhow::context::impl$0::with_context<ddsfile::Dds,enum2$<ddsfile::error::Error>,alloc::string::String,asset_pipe::texture::process_dds_image::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<ddsfile::Dds,enum2$<ddsfile::error::Error>,alloc::string::String,asset_pipe::texture::process_dds_image::closure_env$1>; module=3165, inline_record=4132; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::process_dds_image::closure$2
// 信息 ID: 2efc0a615322a1f984c2a4f4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "asset_pipe::texture::process_dds_image", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture::compile_texture_asset; module=1727, inline_record=41728; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::process_dds_image::closure$3
// 信息 ID: 76614a2d9d7b471c9910f230
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "asset_pipe::texture::process_dds_image", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture::compile_texture_asset; module=1727, inline_record=41840; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::process_image_array
// 信息 ID: 83bbd2e4794b3a40a86e7b15
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture::compile_texture_asset; module=1727, inline_record=33880; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::process_regular_image
// 信息 ID: 225b68fbb3e9fae2e73cbb45
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset_pipe::texture::compile_texture_asset @ 0x17b99fc -> 0x17bbc10 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset_pipe::texture::compile_texture_asset @ 0x17ba018 -> 0x17bbc10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::process_regular_image::closure$0
// 信息 ID: 26d632ce7895bda9514959c9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "asset_pipe::texture::process_regular_image", "captures": "unknown"}
// [闭包环境类型引用] asset_pipe::texture::process_regular_image::closure_env$0 <- anyhow::context::impl$0::with_context<enum2$<image::dynimage::DynamicImage>,enum2$<image::error::ImageError>,alloc::string::String,asset_pipe::texture::process_regular_image::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<enum2$<image::dynimage::DynamicImage>,enum2$<image::error::ImageError>,alloc::string::String,asset_pipe::texture::process_regular_image::closure_env$0>; module=3165, inline_record=3476; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::process_regular_image::closure$2
// 信息 ID: 43e5f0894bc862deec684b39
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "asset_pipe::texture::process_regular_image", "captures": "unknown"}
// [闭包环境类型引用] asset_pipe::texture::process_regular_image::closure_env$2 <- asset_pipe::texture::with_rgba8<asset_pipe::texture::process_regular_image::closure_env$2,alloc::vec::Vec<u8,alloc::alloc::Global> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] asset_pipe::texture::with_rgba8<asset_pipe::texture::process_regular_image::closure_env$2,alloc::vec::Vec<u8,alloc::alloc::Global> >; module=3187, inline_record=6580; 不是独立调用边。
// [PDB 内联宿主] asset_pipe::texture::with_rgba8<asset_pipe::texture::process_regular_image::closure_env$2,alloc::vec::Vec<u8,alloc::alloc::Global> >; module=3187, inline_record=7584; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::process_regular_image::closure$3
// 信息 ID: c3d639ba091af3b072d8132f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "asset_pipe::texture::process_regular_image", "captures": "unknown"}
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset_pipe::texture::process_regular_image @ 0x17bc17f -> 0x17bc930 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset_pipe::texture::process_regular_image @ 0x17bc21c -> 0x17bc930 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::process_regular_image::closure$4
// 信息 ID: f5b41db9b6e0a2c22ff5a6b9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "asset_pipe::texture::process_regular_image", "captures": "unknown"}
// [闭包环境类型引用] asset_pipe::texture::process_regular_image::closure_env$4 <- asset_pipe::texture::with_rgba8<asset_pipe::texture::process_regular_image::closure_env$4,tuple$<> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] asset_pipe::texture::with_rgba8<asset_pipe::texture::process_regular_image::closure_env$4,tuple$<> >; module=3187, inline_record=2012; 不是独立调用边。
// [PDB 内联宿主] asset_pipe::texture::with_rgba8<asset_pipe::texture::process_regular_image::closure_env$4,tuple$<> >; module=3187, inline_record=3544; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::resize_image
// 信息 ID: 8a5482d03e69ec89a0c1f054
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture::process_regular_image; module=1727, inline_record=50208; 不是独立调用边。
// [PDB 内联宿主] asset_pipe::texture::process_regular_image::closure$3; module=1727, inline_record=51968; 不是独立调用边。
// [PDB 内联宿主] asset_pipe::texture::process_regular_image::closure$3; module=1727, inline_record=52164; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::srgb_eotf
// 信息 ID: f7031b72ec2bb243b480e6ac
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture::process_regular_image; module=1727, inline_record=49752; 不是独立调用边。
// [PDB 内联宿主] asset_pipe::texture::process_regular_image; module=1727, inline_record=49820; 不是独立调用边。
// [PDB 内联宿主] asset_pipe::texture::process_regular_image; module=1727, inline_record=49884; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::srgb_oetf
// 信息 ID: d78c66ec874abe562ab507b9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset_pipe::texture::with_rgba8<asset_pipe::texture::process_regular_image::closure_env$4,tuple$<> > @ 0x20e2c48 -> 0x17bcd20 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset_pipe::texture::with_rgba8<asset_pipe::texture::process_regular_image::closure_env$4,tuple$<> > @ 0x20e2c5a -> 0x17bcd20 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset_pipe::texture::with_rgba8<asset_pipe::texture::process_regular_image::closure_env$4,tuple$<> > @ 0x20e2c6d -> 0x17bcd20 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset_pipe::texture::with_rgba8<asset_pipe::texture::process_regular_image::closure_env$2,alloc::vec::Vec<u8,alloc::alloc::Global> > @ 0x20e30f9 -> 0x17bcd20 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset_pipe::texture::with_rgba8<asset_pipe::texture::process_regular_image::closure_env$2,alloc::vec::Vec<u8,alloc::alloc::Global> > @ 0x20e310d -> 0x17bcd20 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset_pipe::texture::with_rgba8<asset_pipe::texture::process_regular_image::closure_env$2,alloc::vec::Vec<u8,alloc::alloc::Global> > @ 0x20e3122 -> 0x17bcd20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::with_rgba8<asset_pipe::texture::process_regular_image::closure_env$2,alloc::vec::Vec<u8,alloc::alloc::Global> >
// 信息 ID: 084fbab3016199255daaf50d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["u8", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] asset_pipe::texture::with_rgba8 -> ["asset_pipe::texture::process_regular_image::closure_env$2", "alloc::vec::Vec<u8,alloc::alloc::Global>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset_pipe::texture::process_regular_image @ 0x17bc152 -> 0x20e3080 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::with_rgba8<asset_pipe::texture::process_regular_image::closure_env$4,tuple$<> >
// 信息 ID: 4ba392f13f28f14d5fc7e0ff
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tuple$ -> []; 字段布局 = unknown。
// [类型关系] asset_pipe::texture::with_rgba8 -> ["asset_pipe::texture::process_regular_image::closure_env$4", "tuple$<>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset_pipe::texture::process_regular_image @ 0x17bc203 -> 0x20e2bd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
