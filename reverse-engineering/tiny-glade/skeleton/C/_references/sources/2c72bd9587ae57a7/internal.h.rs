// Generated from PDB metadata by skeleton/_tools/fill_type_structures.py.
// Version: C
// 类型元数据；原作者源码文本/定义文件见逐项映射；不生成依赖实现。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: internal.h.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB forward_declaration_only] fe; TPI 0x1e72
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "fe", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [7800]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] fe; TPI 0x1e78
// 原始定义文件映射: PDB_UDT_source_line
// size = 40; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 1, "Options": ["None"], "FieldList": 7799, "Name": "fe", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 40}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7799, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7798, "FieldOffset": 0, "Name": "v"}}, "type_references": [{"index": 7798, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 35, "IndexType": 35, "Size": 40, "Name": ""}}, "nested_types": {"ElementType": {"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [PDB UDT 源码行] {"ipi_index": 138458, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ring-0.16.20/crypto/curve25519/internal.h", "line": 56, "module": null, "source_string_id": 138457}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] fe_loose; TPI 0x1e87
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "fe_loose", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [7819]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] fe_loose; TPI 0x1e8b
// 原始定义文件映射: PDB_UDT_source_line
// size = 40; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 1, "Options": ["None"], "FieldList": 7799, "Name": "fe_loose", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 40}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7799, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7798, "FieldOffset": 0, "Name": "v"}}, "type_references": [{"index": 7798, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 35, "IndexType": 35, "Size": 40, "Name": ""}}, "nested_types": {"ElementType": {"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [PDB UDT 源码行] {"ipi_index": 138468, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ring-0.16.20/crypto/curve25519/internal.h", "line": 60, "module": null, "source_string_id": 138457}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] ge_p3; TPI 0x1e9a
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ge_p3", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [7839]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] ge_p3; TPI 0x1e9f
// 原始定义文件映射: PDB_UDT_source_line
// size = 160; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 7838, "Name": "ge_p3", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 160}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7838, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7794, "FieldOffset": 0, "Name": "X"}}, "type_references": [{"index": 7794, "kind": "LF_STRUCTURE", "spelling": "fe", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7838, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7794, "FieldOffset": 40, "Name": "Y"}}, "type_references": [{"index": 7794, "kind": "LF_STRUCTURE", "spelling": "fe", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7838, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7794, "FieldOffset": 80, "Name": "Z"}}, "type_references": [{"index": 7794, "kind": "LF_STRUCTURE", "spelling": "fe", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7838, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7794, "FieldOffset": 120, "Name": "T"}}, "type_references": [{"index": 7794, "kind": "LF_STRUCTURE", "spelling": "fe", "status": "PDB_record_name"}]}
// [PDB UDT 源码行] {"ipi_index": 138482, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ring-0.16.20/crypto/curve25519/internal.h", "line": 89, "module": null, "source_string_id": 138457}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] ge_p1p1; TPI 0x1ea0
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ge_p1p1", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [7846]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] ge_p1p1; TPI 0x1ea6
// 原始定义文件映射: PDB_UDT_source_line
// size = 160; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 7845, "Name": "ge_p1p1", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 160}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7845, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7815, "FieldOffset": 0, "Name": "X"}}, "type_references": [{"index": 7815, "kind": "LF_STRUCTURE", "spelling": "fe_loose", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7845, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7815, "FieldOffset": 40, "Name": "Y"}}, "type_references": [{"index": 7815, "kind": "LF_STRUCTURE", "spelling": "fe_loose", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7845, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7815, "FieldOffset": 80, "Name": "Z"}}, "type_references": [{"index": 7815, "kind": "LF_STRUCTURE", "spelling": "fe_loose", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7845, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7815, "FieldOffset": 120, "Name": "T"}}, "type_references": [{"index": 7815, "kind": "LF_STRUCTURE", "spelling": "fe_loose", "status": "PDB_record_name"}]}
// [PDB UDT 源码行] {"ipi_index": 138485, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ring-0.16.20/crypto/curve25519/internal.h", "line": 96, "module": null, "source_string_id": 138457}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] ge_p2; TPI 0x1eae
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ge_p2", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [7859]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] ge_p2; TPI 0x1eb3
// 原始定义文件映射: PDB_UDT_source_line
// size = 120; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None"], "FieldList": 7858, "Name": "ge_p2", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 120}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7858, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7794, "FieldOffset": 0, "Name": "X"}}, "type_references": [{"index": 7794, "kind": "LF_STRUCTURE", "spelling": "fe", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7858, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7794, "FieldOffset": 40, "Name": "Y"}}, "type_references": [{"index": 7794, "kind": "LF_STRUCTURE", "spelling": "fe", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7858, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7794, "FieldOffset": 80, "Name": "Z"}}, "type_references": [{"index": 7794, "kind": "LF_STRUCTURE", "spelling": "fe", "status": "PDB_record_name"}]}
// [PDB UDT 源码行] {"ipi_index": 138489, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ring-0.16.20/crypto/curve25519/internal.h", "line": 81, "module": null, "source_string_id": 138457}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] ge_precomp; TPI 0x1eb6
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ge_precomp", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [7867]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] ge_precomp; TPI 0x1ebb
// 原始定义文件映射: PDB_UDT_source_line
// size = 120; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None"], "FieldList": 7866, "Name": "ge_precomp", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 120}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7866, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7815, "FieldOffset": 0, "Name": "yplusx"}}, "type_references": [{"index": 7815, "kind": "LF_STRUCTURE", "spelling": "fe_loose", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7866, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7815, "FieldOffset": 40, "Name": "yminusx"}}, "type_references": [{"index": 7815, "kind": "LF_STRUCTURE", "spelling": "fe_loose", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7866, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7815, "FieldOffset": 80, "Name": "xy2d"}}, "type_references": [{"index": 7815, "kind": "LF_STRUCTURE", "spelling": "fe_loose", "status": "PDB_record_name"}]}
// [PDB UDT 源码行] {"ipi_index": 138494, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ring-0.16.20/crypto/curve25519/internal.h", "line": 103, "module": null, "source_string_id": 138457}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] ge_cached; TPI 0x1eda
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ge_cached", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [7903]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] ge_cached; TPI 0x1edf
// 原始定义文件映射: PDB_UDT_source_line
// size = 160; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 7902, "Name": "ge_cached", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 160}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7902, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7815, "FieldOffset": 0, "Name": "YplusX"}}, "type_references": [{"index": 7815, "kind": "LF_STRUCTURE", "spelling": "fe_loose", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7902, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7815, "FieldOffset": 40, "Name": "YminusX"}}, "type_references": [{"index": 7815, "kind": "LF_STRUCTURE", "spelling": "fe_loose", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7902, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7815, "FieldOffset": 80, "Name": "Z"}}, "type_references": [{"index": 7815, "kind": "LF_STRUCTURE", "spelling": "fe_loose", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7902, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7815, "FieldOffset": 120, "Name": "T2d"}}, "type_references": [{"index": 7815, "kind": "LF_STRUCTURE", "spelling": "fe_loose", "status": "PDB_record_name"}]}
// [PDB UDT 源码行] {"ipi_index": 138519, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ring-0.16.20/crypto/curve25519/internal.h", "line": 109, "module": null, "source_string_id": 138457}
// === 完整 TPI 结构关系引用记录；原文不转换成猜测的 Rust ABI ===
// [TPI 0x1e72] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "fe", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1e76] {"Kind": "LF_ARRAY", "Array": {"ElementType": 35, "IndexType": 35, "Size": 40, "Name": ""}}
// [TPI 0x1e77] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7798, "FieldOffset": 0, "Name": "v"}}]}
// [TPI 0x1e78] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 1, "Options": ["None"], "FieldList": 7799, "Name": "fe", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 40}}
// [TPI 0x1e87] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "fe_loose", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1e8b] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 1, "Options": ["None"], "FieldList": 7799, "Name": "fe_loose", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 40}}
// [TPI 0x1e9a] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ge_p3", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1e9e] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7794, "FieldOffset": 0, "Name": "X"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7794, "FieldOffset": 40, "Name": "Y"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7794, "FieldOffset": 80, "Name": "Z"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7794, "FieldOffset": 120, "Name": "T"}}]}
// [TPI 0x1e9f] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 7838, "Name": "ge_p3", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 160}}
// [TPI 0x1ea0] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ge_p1p1", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1ea5] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7815, "FieldOffset": 0, "Name": "X"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7815, "FieldOffset": 40, "Name": "Y"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7815, "FieldOffset": 80, "Name": "Z"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7815, "FieldOffset": 120, "Name": "T"}}]}
// [TPI 0x1ea6] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 7845, "Name": "ge_p1p1", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 160}}
// [TPI 0x1eae] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ge_p2", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1eb2] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7794, "FieldOffset": 0, "Name": "X"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7794, "FieldOffset": 40, "Name": "Y"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7794, "FieldOffset": 80, "Name": "Z"}}]}
// [TPI 0x1eb3] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None"], "FieldList": 7858, "Name": "ge_p2", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 120}}
// [TPI 0x1eb6] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ge_precomp", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1eba] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7815, "FieldOffset": 0, "Name": "yplusx"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7815, "FieldOffset": 40, "Name": "yminusx"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7815, "FieldOffset": 80, "Name": "xy2d"}}]}
// [TPI 0x1ebb] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None"], "FieldList": 7866, "Name": "ge_precomp", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 120}}
// [TPI 0x1eda] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ge_cached", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1ede] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7815, "FieldOffset": 0, "Name": "YplusX"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7815, "FieldOffset": 40, "Name": "YminusX"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7815, "FieldOffset": 80, "Name": "Z"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7815, "FieldOffset": 120, "Name": "T2d"}}]}
// [TPI 0x1edf] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 7902, "Name": "ge_cached", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 160}}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: internal.h.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===
