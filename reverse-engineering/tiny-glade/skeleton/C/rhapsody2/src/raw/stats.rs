// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: rhapsody2/src/raw/stats.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: f5d076dc843b31a2c62a832440a2599c2cb9f5314709f9e09a0c02fa2cfd0de3
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: stats.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 202, 798

// 归属: dependency_reuse_metadata_only
// Repository: PounceLight/rhapsody2
// Revision: 48efa87114d22e484592a0e9efe866faafa6a876
// Source kind: repository_source

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::stats::impl$2::default
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 3
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xCA07
// 首条位置: {"module":798,"inlinee":51719}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::stats::RenderStats::snapshot_and_reset
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 11
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F46
// 首条位置: {"module":202,"inlinee":16198}
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
// IPI 0x10AA:
// ---
// Kind:            LF_STRING_ID
// StringId:
//   Id:              0
//   String:          '/home/h3/code/pouncelight/country-slice-private'
// ...
// IPI 0x10AC:
// ---
// Kind:            LF_STRING_ID
// StringId:
//   Id:              0
//   String:          ''
// ...
// IPI 0x3F45:
// ---
// Kind:            LF_STRING_ID
// StringId:
//   Id:              0
//   String:          'rhapsody::raw::stats::RenderStats'
// ...
// IPI 0x3F46:
// ---
// Kind:            LF_FUNC_ID
// FuncId:
//   ParentScope:     16197
//   FunctionType:    4097
//   Name:            snapshot_and_reset
// ...
// IPI 0x3F51:
// ---
// Kind:            LF_STRING_ID
// StringId:
//   Id:              0
//   String:          'crates/tiny-glade/src/main.rs/@/bjxsfqd183z0r43mmz4adl07r'
// ...
// IPI 0x3F52:
// ---
// Kind:            LF_BUILDINFO
// BuildInfo:
//   ArgIndices:      [ 4266, 4268, 16209, 4268, 4268 ]
// ...
// IPI 0xC543:
// ---
// Kind:            LF_STRING_ID
// StringId:
//   Id:              0
//   String:          '/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87'
// ...
// IPI 0xCA06:
// ---
// Kind:            LF_STRING_ID
// StringId:
//   Id:              0
//   String:          'rhapsody::raw::stats::impl$2'
// ...
// IPI 0xCA07:
// ---
// Kind:            LF_FUNC_ID
// FuncId:
//   ParentScope:     51718
//   FunctionType:    4097
//   Name:            default
// ...
// IPI 0xCAAB:
// ---
// Kind:            LF_STRING_ID
// StringId:
//   Id:              0
//   String:          '/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/lib.rs/@/rhapsody.73b8a89e5e911c82-cgu.00'
// ...
// IPI 0xCAAC:
// ---
// Kind:            LF_BUILDINFO
// BuildInfo:
//   ArgIndices:      [ 50499, 4268, 51883, 4268, 4268 ]
// ...

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: stats.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::stats::RenderStats
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 0
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: stats.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===
