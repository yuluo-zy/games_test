// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: rhapsody2/crates/dxgi-enumerate-gpus/src/lib.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 37409d5a7eec2db599e19c61724520810d98e8309977176ebb8b5ddb5412f5ff
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 2234

// 归属: dependency_reuse_metadata_only
// Repository: PounceLight/rhapsody2
// Revision: 48efa87114d22e484592a0e9efe866faafa6a876
// Source kind: repository_source

// ------------------------------------------------------------------
// 函数符号: dxgi_enumerate_gpus::enumerate
// 记录形式: compiled_procedure; instances=1
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2234,"record_offset":284,"rva":31903824,"va":5400612944}
// 编译实例 RVAs: 0x1e6d050
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// ------------------------------------------------------------------
// 函数符号: dxgi_enumerate_gpus::get_string_from_wide
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x186B9
// 首条位置: {"module":2234,"inlinee":100025}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// ------------------------------------------------------------------
// 函数符号: dxgi_enumerate_gpus::get_string_from_wide::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 37
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x186BB
// 首条位置: {"module":2234,"inlinee":100027}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

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
// === 可用 IPI 函数身份、作用域字符串与构建信息引用图 ===
// IPI 0x10AC:
// ---
// Kind:            LF_STRING_ID
// StringId:
//   Id:              0
//   String:          ''
// ...
// IPI 0x186B8:
// ---
// Kind:            LF_STRING_ID
// StringId:
//   Id:              0
//   String:          dxgi_enumerate_gpus
// ...
// IPI 0x186B9:
// ---
// Kind:            LF_FUNC_ID
// FuncId:
//   ParentScope:     100024
//   FunctionType:    4097
//   Name:            get_string_from_wide
// ...
// IPI 0x186BA:
// ---
// Kind:            LF_STRING_ID
// StringId:
//   Id:              0
//   String:          'dxgi_enumerate_gpus::get_string_from_wide'
// ...
// IPI 0x186BB:
// ---
// Kind:            LF_FUNC_ID
// FuncId:
//   ParentScope:     100026
//   FunctionType:    4097
//   Name:            'closure$0'
// ...
// IPI 0x186BD:
// ---
// Kind:            LF_STRING_ID
// StringId:
//   Id:              0
//   String:          '/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/crates/dxgi-enumerate-gpus'
// ...
// IPI 0x186BE:
// ---
// Kind:            LF_STRING_ID
// StringId:
//   Id:              0
//   String:          '/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/crates/dxgi-enumerate-gpus/src/lib.rs/@/dxgi_enumerate_gpus.85ad8e5fc45b45c6-cgu.0'
// ...
// IPI 0x186BF:
// ---
// Kind:            LF_BUILDINFO
// BuildInfo:
//   ArgIndices:      [ 100029, 4268, 100030, 4268, 4268 ]
// ...

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: lib.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] dxgi_enumerate_gpus::DxgiAdapter
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 5
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<dxgi_enumerate_gpus::DxgiAdapter,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["dxgi_enumerate_gpus::DxgiAdapter", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<alloc::vec::Vec<dxgi_enumerate_gpus::DxgiAdapter,alloc::alloc::Global>,anyhow::Error>", "constructor": "core::result::Result", "arguments": ["alloc::vec::Vec<dxgi_enumerate_gpus::DxgiAdapter,alloc::alloc::Global>", "anyhow::Error"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<alloc::vec::Vec<dxgi_enumerate_gpus::DxgiAdapter,alloc::alloc::Global>,anyhow::Error> >", "constructor": "enum2$", "arguments": ["core::result::Result<alloc::vec::Vec<dxgi_enumerate_gpus::DxgiAdapter,alloc::alloc::Global>,anyhow::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<alloc::vec::Vec<dxgi_enumerate_gpus::DxgiAdapter,alloc::alloc::Global>,anyhow::Error> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<alloc::vec::Vec<dxgi_enumerate_gpus::DxgiAdapter,alloc::alloc::Global>,anyhow::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<dxgi_enumerate_gpus::DxgiAdapter,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<dxgi_enumerate_gpus::DxgiAdapter,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::impl$16::drop<dxgi_enumerate_gpus::DxgiAdapter,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::impl$16::drop", "arguments": ["dxgi_enumerate_gpus::DxgiAdapter", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<dxgi_enumerate_gpus::DxgiAdapter,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["dxgi_enumerate_gpus::DxgiAdapter", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: lib.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===
