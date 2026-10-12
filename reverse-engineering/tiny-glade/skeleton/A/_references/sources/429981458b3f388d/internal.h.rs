// Generated from PDB metadata by skeleton/_tools/fill_type_structures.py.
// Version: A
// 类型元数据；原作者源码文本/定义文件见逐项映射；不生成依赖实现。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: internal.h.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB forward_declaration_only] fe; TPI 0x1c82
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "fe", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [7304]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] fe; TPI 0x1c88
// 原始定义文件映射: PDB_UDT_source_line
// size = 40; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 1, "Options": ["None"], "FieldList": 7303, "Name": "fe", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 40}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7303, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7302, "FieldOffset": 0, "Name": "v"}}, "type_references": [{"index": 7302, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 35, "IndexType": 35, "Size": 40, "Name": ""}}, "nested_types": {"ElementType": {"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [PDB UDT 源码行] {"ipi_index": 111359, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ring-0.16.20/crypto/curve25519/internal.h", "line": 56, "module": null, "source_string_id": 111358}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] fe_loose; TPI 0x1c91
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "fe_loose", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [7317]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] fe_loose; TPI 0x1c95
// 原始定义文件映射: PDB_UDT_source_line
// size = 40; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 1, "Options": ["None"], "FieldList": 7303, "Name": "fe_loose", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 40}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7303, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7302, "FieldOffset": 0, "Name": "v"}}, "type_references": [{"index": 7302, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 35, "IndexType": 35, "Size": 40, "Name": ""}}, "nested_types": {"ElementType": {"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [PDB UDT 源码行] {"ipi_index": 111366, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ring-0.16.20/crypto/curve25519/internal.h", "line": 60, "module": null, "source_string_id": 111358}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] ge_p3; TPI 0x1ca4
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ge_p3", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [7337]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] ge_p3; TPI 0x1ca9
// 原始定义文件映射: PDB_UDT_source_line
// size = 160; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 7336, "Name": "ge_p3", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 160}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7336, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7298, "FieldOffset": 0, "Name": "X"}}, "type_references": [{"index": 7298, "kind": "LF_STRUCTURE", "spelling": "fe", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7336, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7298, "FieldOffset": 40, "Name": "Y"}}, "type_references": [{"index": 7298, "kind": "LF_STRUCTURE", "spelling": "fe", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7336, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7298, "FieldOffset": 80, "Name": "Z"}}, "type_references": [{"index": 7298, "kind": "LF_STRUCTURE", "spelling": "fe", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7336, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7298, "FieldOffset": 120, "Name": "T"}}, "type_references": [{"index": 7298, "kind": "LF_STRUCTURE", "spelling": "fe", "status": "PDB_record_name"}]}
// [PDB UDT 源码行] {"ipi_index": 111381, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ring-0.16.20/crypto/curve25519/internal.h", "line": 89, "module": null, "source_string_id": 111358}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] ge_p1p1; TPI 0x1caa
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ge_p1p1", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [7344]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] ge_p1p1; TPI 0x1cb0
// 原始定义文件映射: PDB_UDT_source_line
// size = 160; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 7343, "Name": "ge_p1p1", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 160}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7343, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7313, "FieldOffset": 0, "Name": "X"}}, "type_references": [{"index": 7313, "kind": "LF_STRUCTURE", "spelling": "fe_loose", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7343, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7313, "FieldOffset": 40, "Name": "Y"}}, "type_references": [{"index": 7313, "kind": "LF_STRUCTURE", "spelling": "fe_loose", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7343, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7313, "FieldOffset": 80, "Name": "Z"}}, "type_references": [{"index": 7313, "kind": "LF_STRUCTURE", "spelling": "fe_loose", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7343, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7313, "FieldOffset": 120, "Name": "T"}}, "type_references": [{"index": 7313, "kind": "LF_STRUCTURE", "spelling": "fe_loose", "status": "PDB_record_name"}]}
// [PDB UDT 源码行] {"ipi_index": 111384, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ring-0.16.20/crypto/curve25519/internal.h", "line": 96, "module": null, "source_string_id": 111358}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] ge_p2; TPI 0x1cb8
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ge_p2", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [7357]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] ge_p2; TPI 0x1cbd
// 原始定义文件映射: PDB_UDT_source_line
// size = 120; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None"], "FieldList": 7356, "Name": "ge_p2", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 120}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7356, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7298, "FieldOffset": 0, "Name": "X"}}, "type_references": [{"index": 7298, "kind": "LF_STRUCTURE", "spelling": "fe", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7356, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7298, "FieldOffset": 40, "Name": "Y"}}, "type_references": [{"index": 7298, "kind": "LF_STRUCTURE", "spelling": "fe", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7356, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7298, "FieldOffset": 80, "Name": "Z"}}, "type_references": [{"index": 7298, "kind": "LF_STRUCTURE", "spelling": "fe", "status": "PDB_record_name"}]}
// [PDB UDT 源码行] {"ipi_index": 111388, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ring-0.16.20/crypto/curve25519/internal.h", "line": 81, "module": null, "source_string_id": 111358}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] ge_precomp; TPI 0x1cc2
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ge_precomp", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [7367]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] ge_precomp; TPI 0x1cc7
// 原始定义文件映射: PDB_UDT_source_line
// size = 120; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None"], "FieldList": 7366, "Name": "ge_precomp", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 120}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7366, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7313, "FieldOffset": 0, "Name": "yplusx"}}, "type_references": [{"index": 7313, "kind": "LF_STRUCTURE", "spelling": "fe_loose", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7366, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7313, "FieldOffset": 40, "Name": "yminusx"}}, "type_references": [{"index": 7313, "kind": "LF_STRUCTURE", "spelling": "fe_loose", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7366, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7313, "FieldOffset": 80, "Name": "xy2d"}}, "type_references": [{"index": 7313, "kind": "LF_STRUCTURE", "spelling": "fe_loose", "status": "PDB_record_name"}]}
// [PDB UDT 源码行] {"ipi_index": 111394, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ring-0.16.20/crypto/curve25519/internal.h", "line": 103, "module": null, "source_string_id": 111358}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] ge_cached; TPI 0x1cdc
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ge_cached", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [7393]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] ge_cached; TPI 0x1ce1
// 原始定义文件映射: PDB_UDT_source_line
// size = 160; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 7392, "Name": "ge_cached", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 160}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7392, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7313, "FieldOffset": 0, "Name": "YplusX"}}, "type_references": [{"index": 7313, "kind": "LF_STRUCTURE", "spelling": "fe_loose", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7392, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7313, "FieldOffset": 40, "Name": "YminusX"}}, "type_references": [{"index": 7313, "kind": "LF_STRUCTURE", "spelling": "fe_loose", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7392, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7313, "FieldOffset": 80, "Name": "Z"}}, "type_references": [{"index": 7313, "kind": "LF_STRUCTURE", "spelling": "fe_loose", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7392, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7313, "FieldOffset": 120, "Name": "T2d"}}, "type_references": [{"index": 7313, "kind": "LF_STRUCTURE", "spelling": "fe_loose", "status": "PDB_record_name"}]}
// [PDB UDT 源码行] {"ipi_index": 111417, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ring-0.16.20/crypto/curve25519/internal.h", "line": 109, "module": null, "source_string_id": 111358}
// === 完整 TPI 结构关系引用记录；原文不转换成猜测的 Rust ABI ===
// [TPI 0x1c82] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "fe", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1c86] {"Kind": "LF_ARRAY", "Array": {"ElementType": 35, "IndexType": 35, "Size": 40, "Name": ""}}
// [TPI 0x1c87] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7302, "FieldOffset": 0, "Name": "v"}}]}
// [TPI 0x1c88] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 1, "Options": ["None"], "FieldList": 7303, "Name": "fe", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 40}}
// [TPI 0x1c91] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "fe_loose", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1c95] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 1, "Options": ["None"], "FieldList": 7303, "Name": "fe_loose", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 40}}
// [TPI 0x1ca4] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ge_p3", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1ca8] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7298, "FieldOffset": 0, "Name": "X"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7298, "FieldOffset": 40, "Name": "Y"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7298, "FieldOffset": 80, "Name": "Z"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7298, "FieldOffset": 120, "Name": "T"}}]}
// [TPI 0x1ca9] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 7336, "Name": "ge_p3", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 160}}
// [TPI 0x1caa] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ge_p1p1", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1caf] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7313, "FieldOffset": 0, "Name": "X"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7313, "FieldOffset": 40, "Name": "Y"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7313, "FieldOffset": 80, "Name": "Z"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7313, "FieldOffset": 120, "Name": "T"}}]}
// [TPI 0x1cb0] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 7343, "Name": "ge_p1p1", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 160}}
// [TPI 0x1cb8] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ge_p2", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1cbc] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7298, "FieldOffset": 0, "Name": "X"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7298, "FieldOffset": 40, "Name": "Y"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7298, "FieldOffset": 80, "Name": "Z"}}]}
// [TPI 0x1cbd] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None"], "FieldList": 7356, "Name": "ge_p2", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 120}}
// [TPI 0x1cc2] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ge_precomp", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1cc6] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7313, "FieldOffset": 0, "Name": "yplusx"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7313, "FieldOffset": 40, "Name": "yminusx"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7313, "FieldOffset": 80, "Name": "xy2d"}}]}
// [TPI 0x1cc7] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None"], "FieldList": 7366, "Name": "ge_precomp", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 120}}
// [TPI 0x1cdc] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ge_cached", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1ce0] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7313, "FieldOffset": 0, "Name": "YplusX"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7313, "FieldOffset": 40, "Name": "YminusX"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7313, "FieldOffset": 80, "Name": "Z"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7313, "FieldOffset": 120, "Name": "T2d"}}]}
// [TPI 0x1ce1] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 7392, "Name": "ge_cached", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 160}}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: internal.h.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===
