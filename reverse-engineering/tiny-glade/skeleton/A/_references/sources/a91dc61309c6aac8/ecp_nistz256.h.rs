// Generated from PDB metadata by skeleton/_tools/fill_type_structures.py.
// Version: A
// 类型元数据；原作者源码文本/定义文件见逐项映射；不生成依赖实现。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: ecp_nistz256.h.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB forward_declaration_only] P256_POINT_AFFINE; TPI 0x22c7
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "P256_POINT_AFFINE", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [8908]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] P256_POINT_AFFINE; TPI 0x22cc
// 原始定义文件映射: PDB_UDT_source_line
// size = 64; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 8907, "Name": "P256_POINT_AFFINE", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 64}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8907, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6225, "FieldOffset": 0, "Name": "X"}}, "type_references": [{"index": 6225, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 35, "IndexType": 35, "Size": 32, "Name": ""}}, "nested_types": {"ElementType": {"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8907, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6225, "FieldOffset": 32, "Name": "Y"}}, "type_references": [{"index": 6225, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 35, "IndexType": 35, "Size": 32, "Name": ""}}, "nested_types": {"ElementType": {"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [PDB UDT 源码行] {"ipi_index": 120771, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ring-0.16.20/crypto/fipsmodule/ec/ecp_nistz256.h", "line": 34, "module": null, "source_string_id": 120770}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] P256_POINT; TPI 0x22cf
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "P256_POINT", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [8916]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] P256_POINT; TPI 0x22d4
// 原始定义文件映射: PDB_UDT_source_line
// size = 96; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None"], "FieldList": 8915, "Name": "P256_POINT", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 96}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8915, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6225, "FieldOffset": 0, "Name": "X"}}, "type_references": [{"index": 6225, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 35, "IndexType": 35, "Size": 32, "Name": ""}}, "nested_types": {"ElementType": {"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8915, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6225, "FieldOffset": 32, "Name": "Y"}}, "type_references": [{"index": 6225, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 35, "IndexType": 35, "Size": 32, "Name": ""}}, "nested_types": {"ElementType": {"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8915, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6225, "FieldOffset": 64, "Name": "Z"}}, "type_references": [{"index": 6225, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 35, "IndexType": 35, "Size": 32, "Name": ""}}, "nested_types": {"ElementType": {"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [PDB UDT 源码行] {"ipi_index": 120774, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ring-0.16.20/crypto/fipsmodule/ec/ecp_nistz256.h", "line": 27, "module": null, "source_string_id": 120770}
// === 完整 TPI 结构关系引用记录；原文不转换成猜测的 Rust ABI ===
// [TPI 0x1851] {"Kind": "LF_ARRAY", "Array": {"ElementType": 35, "IndexType": 35, "Size": 32, "Name": ""}}
// [TPI 0x22c7] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "P256_POINT_AFFINE", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x22cb] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6225, "FieldOffset": 0, "Name": "X"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6225, "FieldOffset": 32, "Name": "Y"}}]}
// [TPI 0x22cc] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 8907, "Name": "P256_POINT_AFFINE", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 64}}
// [TPI 0x22cf] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "P256_POINT", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x22d3] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6225, "FieldOffset": 0, "Name": "X"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6225, "FieldOffset": 32, "Name": "Y"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6225, "FieldOffset": 64, "Name": "Z"}}]}
// [TPI 0x22d4] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None"], "FieldList": 8915, "Name": "P256_POINT", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 96}}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: ecp_nistz256.h.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===
