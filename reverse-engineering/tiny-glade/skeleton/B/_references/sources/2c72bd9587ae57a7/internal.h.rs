// Generated from PDB metadata by skeleton/_tools/fill_type_structures.py.
// Version: B
// 类型元数据；原作者源码文本/定义文件见逐项映射；不生成依赖实现。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: internal.h.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB forward_declaration_only] fe; TPI 0x19f5
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "fe", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [6651]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] fe; TPI 0x19fb
// 原始定义文件映射: PDB_UDT_source_line
// size = 40; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 1, "Options": ["None"], "FieldList": 6650, "Name": "fe", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 40}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6650, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6649, "FieldOffset": 0, "Name": "v"}}, "type_references": [{"index": 6649, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 35, "IndexType": 35, "Size": 40, "Name": ""}}, "nested_types": {"ElementType": {"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [PDB UDT 源码行] {"ipi_index": 124580, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ring-0.16.20/crypto/curve25519/internal.h", "line": 56, "module": null, "source_string_id": 124579}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] fe_loose; TPI 0x1a0a
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "fe_loose", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [6670]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] fe_loose; TPI 0x1a0e
// 原始定义文件映射: PDB_UDT_source_line
// size = 40; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 1, "Options": ["None"], "FieldList": 6650, "Name": "fe_loose", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 40}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6650, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6649, "FieldOffset": 0, "Name": "v"}}, "type_references": [{"index": 6649, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 35, "IndexType": 35, "Size": 40, "Name": ""}}, "nested_types": {"ElementType": {"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [PDB UDT 源码行] {"ipi_index": 124590, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ring-0.16.20/crypto/curve25519/internal.h", "line": 60, "module": null, "source_string_id": 124579}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] ge_p3; TPI 0x1a1d
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ge_p3", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [6690]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] ge_p3; TPI 0x1a22
// 原始定义文件映射: PDB_UDT_source_line
// size = 160; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 6689, "Name": "ge_p3", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 160}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6689, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6645, "FieldOffset": 0, "Name": "X"}}, "type_references": [{"index": 6645, "kind": "LF_STRUCTURE", "spelling": "fe", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6689, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6645, "FieldOffset": 40, "Name": "Y"}}, "type_references": [{"index": 6645, "kind": "LF_STRUCTURE", "spelling": "fe", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6689, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6645, "FieldOffset": 80, "Name": "Z"}}, "type_references": [{"index": 6645, "kind": "LF_STRUCTURE", "spelling": "fe", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6689, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6645, "FieldOffset": 120, "Name": "T"}}, "type_references": [{"index": 6645, "kind": "LF_STRUCTURE", "spelling": "fe", "status": "PDB_record_name"}]}
// [PDB UDT 源码行] {"ipi_index": 124604, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ring-0.16.20/crypto/curve25519/internal.h", "line": 89, "module": null, "source_string_id": 124579}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] ge_p1p1; TPI 0x1a23
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ge_p1p1", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [6697]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] ge_p1p1; TPI 0x1a29
// 原始定义文件映射: PDB_UDT_source_line
// size = 160; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 6696, "Name": "ge_p1p1", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 160}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6696, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6666, "FieldOffset": 0, "Name": "X"}}, "type_references": [{"index": 6666, "kind": "LF_STRUCTURE", "spelling": "fe_loose", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6696, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6666, "FieldOffset": 40, "Name": "Y"}}, "type_references": [{"index": 6666, "kind": "LF_STRUCTURE", "spelling": "fe_loose", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6696, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6666, "FieldOffset": 80, "Name": "Z"}}, "type_references": [{"index": 6666, "kind": "LF_STRUCTURE", "spelling": "fe_loose", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6696, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6666, "FieldOffset": 120, "Name": "T"}}, "type_references": [{"index": 6666, "kind": "LF_STRUCTURE", "spelling": "fe_loose", "status": "PDB_record_name"}]}
// [PDB UDT 源码行] {"ipi_index": 124607, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ring-0.16.20/crypto/curve25519/internal.h", "line": 96, "module": null, "source_string_id": 124579}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] ge_p2; TPI 0x1a31
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ge_p2", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [6710]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] ge_p2; TPI 0x1a36
// 原始定义文件映射: PDB_UDT_source_line
// size = 120; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None"], "FieldList": 6709, "Name": "ge_p2", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 120}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6709, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6645, "FieldOffset": 0, "Name": "X"}}, "type_references": [{"index": 6645, "kind": "LF_STRUCTURE", "spelling": "fe", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6709, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6645, "FieldOffset": 40, "Name": "Y"}}, "type_references": [{"index": 6645, "kind": "LF_STRUCTURE", "spelling": "fe", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6709, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6645, "FieldOffset": 80, "Name": "Z"}}, "type_references": [{"index": 6645, "kind": "LF_STRUCTURE", "spelling": "fe", "status": "PDB_record_name"}]}
// [PDB UDT 源码行] {"ipi_index": 124611, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ring-0.16.20/crypto/curve25519/internal.h", "line": 81, "module": null, "source_string_id": 124579}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] ge_precomp; TPI 0x1a39
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ge_precomp", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [6718]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] ge_precomp; TPI 0x1a3e
// 原始定义文件映射: PDB_UDT_source_line
// size = 120; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None"], "FieldList": 6717, "Name": "ge_precomp", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 120}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6717, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6666, "FieldOffset": 0, "Name": "yplusx"}}, "type_references": [{"index": 6666, "kind": "LF_STRUCTURE", "spelling": "fe_loose", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6717, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6666, "FieldOffset": 40, "Name": "yminusx"}}, "type_references": [{"index": 6666, "kind": "LF_STRUCTURE", "spelling": "fe_loose", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6717, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6666, "FieldOffset": 80, "Name": "xy2d"}}, "type_references": [{"index": 6666, "kind": "LF_STRUCTURE", "spelling": "fe_loose", "status": "PDB_record_name"}]}
// [PDB UDT 源码行] {"ipi_index": 124616, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ring-0.16.20/crypto/curve25519/internal.h", "line": 103, "module": null, "source_string_id": 124579}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] ge_cached; TPI 0x1a5d
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ge_cached", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [6754]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] ge_cached; TPI 0x1a62
// 原始定义文件映射: PDB_UDT_source_line
// size = 160; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 6753, "Name": "ge_cached", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 160}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6753, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6666, "FieldOffset": 0, "Name": "YplusX"}}, "type_references": [{"index": 6666, "kind": "LF_STRUCTURE", "spelling": "fe_loose", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6753, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6666, "FieldOffset": 40, "Name": "YminusX"}}, "type_references": [{"index": 6666, "kind": "LF_STRUCTURE", "spelling": "fe_loose", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6753, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6666, "FieldOffset": 80, "Name": "Z"}}, "type_references": [{"index": 6666, "kind": "LF_STRUCTURE", "spelling": "fe_loose", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6753, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6666, "FieldOffset": 120, "Name": "T2d"}}, "type_references": [{"index": 6666, "kind": "LF_STRUCTURE", "spelling": "fe_loose", "status": "PDB_record_name"}]}
// [PDB UDT 源码行] {"ipi_index": 124641, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ring-0.16.20/crypto/curve25519/internal.h", "line": 109, "module": null, "source_string_id": 124579}
// === 完整 TPI 结构关系引用记录；原文不转换成猜测的 Rust ABI ===
// [TPI 0x19f5] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "fe", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x19f9] {"Kind": "LF_ARRAY", "Array": {"ElementType": 35, "IndexType": 35, "Size": 40, "Name": ""}}
// [TPI 0x19fa] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6649, "FieldOffset": 0, "Name": "v"}}]}
// [TPI 0x19fb] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 1, "Options": ["None"], "FieldList": 6650, "Name": "fe", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 40}}
// [TPI 0x1a0a] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "fe_loose", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1a0e] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 1, "Options": ["None"], "FieldList": 6650, "Name": "fe_loose", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 40}}
// [TPI 0x1a1d] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ge_p3", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1a21] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6645, "FieldOffset": 0, "Name": "X"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6645, "FieldOffset": 40, "Name": "Y"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6645, "FieldOffset": 80, "Name": "Z"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6645, "FieldOffset": 120, "Name": "T"}}]}
// [TPI 0x1a22] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 6689, "Name": "ge_p3", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 160}}
// [TPI 0x1a23] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ge_p1p1", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1a28] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6666, "FieldOffset": 0, "Name": "X"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6666, "FieldOffset": 40, "Name": "Y"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6666, "FieldOffset": 80, "Name": "Z"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6666, "FieldOffset": 120, "Name": "T"}}]}
// [TPI 0x1a29] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 6696, "Name": "ge_p1p1", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 160}}
// [TPI 0x1a31] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ge_p2", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1a35] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6645, "FieldOffset": 0, "Name": "X"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6645, "FieldOffset": 40, "Name": "Y"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6645, "FieldOffset": 80, "Name": "Z"}}]}
// [TPI 0x1a36] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None"], "FieldList": 6709, "Name": "ge_p2", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 120}}
// [TPI 0x1a39] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ge_precomp", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1a3d] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6666, "FieldOffset": 0, "Name": "yplusx"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6666, "FieldOffset": 40, "Name": "yminusx"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6666, "FieldOffset": 80, "Name": "xy2d"}}]}
// [TPI 0x1a3e] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None"], "FieldList": 6717, "Name": "ge_precomp", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 120}}
// [TPI 0x1a5d] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ge_cached", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1a61] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6666, "FieldOffset": 0, "Name": "YplusX"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6666, "FieldOffset": 40, "Name": "YminusX"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6666, "FieldOffset": 80, "Name": "Z"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6666, "FieldOffset": 120, "Name": "T2d"}}]}
// [TPI 0x1a62] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 6753, "Name": "ge_cached", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 160}}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: internal.h.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===
