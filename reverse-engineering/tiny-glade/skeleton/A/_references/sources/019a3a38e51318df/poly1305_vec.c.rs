// Generated from PDB metadata by skeleton/_tools/fill_type_structures.py.
// Version: A
// 类型元数据；原作者源码文本/定义文件见逐项映射；不生成依赖实现。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: poly1305_vec.c.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB forward_declaration_only] poly1305_state_internal_t; TPI 0x1c52
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "poly1305_state_internal_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [7261]
// ------------------------------------------------------------------
// [PDB forward_declaration_only] poly1305_power_t; TPI 0x1c58
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "poly1305_power_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [7265]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] poly1305_state_internal_t; TPI 0x1c5d
// 原始定义文件映射: PDB_UDT_source_line
// size = 448; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 6, "Options": ["None"], "FieldList": 7260, "Name": "poly1305_state_internal_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 448}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7260, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7257, "FieldOffset": 0, "Name": "P"}}, "type_references": [{"index": 7257, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 7256, "IndexType": 35, "Size": 288, "Name": ""}}, "nested_types": {"ElementType": {"index": 7256, "kind": "LF_STRUCTURE", "spelling": "poly1305_power_t", "status": "PDB_record_name"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7260, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7258, "FieldOffset": 288, "Name": "H"}}, "type_references": [{"index": 7258, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 6912, "IndexType": 35, "Size": 80, "Name": ""}}, "nested_types": {"ElementType": {"index": 6912, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 19, "IndexType": 35, "Size": 16, "Name": ""}}, "nested_types": {"ElementType": {"index": 19, "spelling": "Int64Quad", "status": "CodeView_simple_type; not Rust source spelling"}}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7260, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7259, "FieldOffset": 288, "Name": "HH"}}, "type_references": [{"index": 7259, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 35, "IndexType": 35, "Size": 80, "Name": ""}}, "nested_types": {"ElementType": {"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7260, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 368, "Name": "started"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7260, "order": 4, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 376, "Name": "leftover"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7260, "order": 5, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7246, "FieldOffset": 384, "Name": "buffer"}}, "type_references": [{"index": 7246, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 32, "IndexType": 35, "Size": 64, "Name": ""}}, "nested_types": {"ElementType": {"index": 32, "spelling": "UnsignedCharacter", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [PDB UDT 源码行] {"ipi_index": 111097, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ring-0.16.20/crypto/poly1305/poly1305_vec.c", "line": 84, "module": null, "source_string_id": 111096}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] poly1305_power_t::<unnamed-tag>; TPI 0x1c5f
// 原始定义文件映射: PDB_UDT_source_line
// size = 16; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_UNION", "Union": {"MemberCount": 3, "Options": ["None", "Nested", "Sealed"], "FieldList": 7262, "Name": "poly1305_power_t::<unnamed-tag>", "UniqueName": "", "Size": 16}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7262, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6912, "FieldOffset": 0, "Name": "v"}}, "type_references": [{"index": 6912, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 19, "IndexType": 35, "Size": 16, "Name": ""}}, "nested_types": {"ElementType": {"index": 19, "spelling": "Int64Quad", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7262, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7224, "FieldOffset": 0, "Name": "u"}}, "type_references": [{"index": 7224, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 35, "IndexType": 35, "Size": 16, "Name": ""}}, "nested_types": {"ElementType": {"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7262, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4768, "FieldOffset": 0, "Name": "d"}}, "type_references": [{"index": 4768, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 117, "IndexType": 35, "Size": 16, "Name": ""}}, "nested_types": {"ElementType": {"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [PDB UDT 源码行] {"ipi_index": 111098, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ring-0.16.20/crypto/poly1305/poly1305_vec.c", "line": 77, "module": null, "source_string_id": 111096}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] poly1305_power_t; TPI 0x1c61
// 原始定义文件映射: PDB_UDT_source_line
// size = 144; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 10, "Options": ["None", "ContainsNestedClass"], "FieldList": 7264, "Name": "poly1305_power_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 144}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7264, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7263, "FieldOffset": 0, "Name": "R20"}}, "type_references": [{"index": 7263, "kind": "LF_UNION", "spelling": "poly1305_power_t::<unnamed-tag>", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7264, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7263, "FieldOffset": 16, "Name": "R21"}}, "type_references": [{"index": 7263, "kind": "LF_UNION", "spelling": "poly1305_power_t::<unnamed-tag>", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7264, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7263, "FieldOffset": 32, "Name": "R22"}}, "type_references": [{"index": 7263, "kind": "LF_UNION", "spelling": "poly1305_power_t::<unnamed-tag>", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7264, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7263, "FieldOffset": 48, "Name": "R23"}}, "type_references": [{"index": 7263, "kind": "LF_UNION", "spelling": "poly1305_power_t::<unnamed-tag>", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7264, "order": 4, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7263, "FieldOffset": 64, "Name": "R24"}}, "type_references": [{"index": 7263, "kind": "LF_UNION", "spelling": "poly1305_power_t::<unnamed-tag>", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7264, "order": 5, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7263, "FieldOffset": 80, "Name": "S21"}}, "type_references": [{"index": 7263, "kind": "LF_UNION", "spelling": "poly1305_power_t::<unnamed-tag>", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7264, "order": 6, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7263, "FieldOffset": 96, "Name": "S22"}}, "type_references": [{"index": 7263, "kind": "LF_UNION", "spelling": "poly1305_power_t::<unnamed-tag>", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7264, "order": 7, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7263, "FieldOffset": 112, "Name": "S23"}}, "type_references": [{"index": 7263, "kind": "LF_UNION", "spelling": "poly1305_power_t::<unnamed-tag>", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7264, "order": 8, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7263, "FieldOffset": 128, "Name": "S24"}}, "type_references": [{"index": 7263, "kind": "LF_UNION", "spelling": "poly1305_power_t::<unnamed-tag>", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7264, "order": 9, "kind": "LF_NESTTYPE", "record": {"Kind": "LF_NESTTYPE", "NestedType": {"Type": 7263, "Name": ""}}, "type_references": [{"index": 7263, "kind": "LF_UNION", "spelling": "poly1305_power_t::<unnamed-tag>", "status": "PDB_record_name"}]}
// [PDB UDT 源码行] {"ipi_index": 111099, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ring-0.16.20/crypto/poly1305/poly1305_vec.c", "line": 76, "module": null, "source_string_id": 111096}
// === 完整 TPI 结构关系引用记录；原文不转换成猜测的 Rust ABI ===
// [TPI 0x12a0] {"Kind": "LF_ARRAY", "Array": {"ElementType": 117, "IndexType": 35, "Size": 16, "Name": ""}}
// [TPI 0x1b00] {"Kind": "LF_ARRAY", "Array": {"ElementType": 19, "IndexType": 35, "Size": 16, "Name": ""}}
// [TPI 0x1c38] {"Kind": "LF_ARRAY", "Array": {"ElementType": 35, "IndexType": 35, "Size": 16, "Name": ""}}
// [TPI 0x1c4e] {"Kind": "LF_ARRAY", "Array": {"ElementType": 32, "IndexType": 35, "Size": 64, "Name": ""}}
// [TPI 0x1c52] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "poly1305_state_internal_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1c58] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "poly1305_power_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1c59] {"Kind": "LF_ARRAY", "Array": {"ElementType": 7256, "IndexType": 35, "Size": 288, "Name": ""}}
// [TPI 0x1c5a] {"Kind": "LF_ARRAY", "Array": {"ElementType": 6912, "IndexType": 35, "Size": 80, "Name": ""}}
// [TPI 0x1c5b] {"Kind": "LF_ARRAY", "Array": {"ElementType": 35, "IndexType": 35, "Size": 80, "Name": ""}}
// [TPI 0x1c5c] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7257, "FieldOffset": 0, "Name": "P"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7258, "FieldOffset": 288, "Name": "H"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7259, "FieldOffset": 288, "Name": "HH"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 368, "Name": "started"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 376, "Name": "leftover"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7246, "FieldOffset": 384, "Name": "buffer"}}]}
// [TPI 0x1c5d] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 6, "Options": ["None"], "FieldList": 7260, "Name": "poly1305_state_internal_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 448}}
// [TPI 0x1c5e] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6912, "FieldOffset": 0, "Name": "v"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7224, "FieldOffset": 0, "Name": "u"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4768, "FieldOffset": 0, "Name": "d"}}]}
// [TPI 0x1c5f] {"Kind": "LF_UNION", "Union": {"MemberCount": 3, "Options": ["None", "Nested", "Sealed"], "FieldList": 7262, "Name": "poly1305_power_t::<unnamed-tag>", "UniqueName": "", "Size": 16}}
// [TPI 0x1c60] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7263, "FieldOffset": 0, "Name": "R20"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7263, "FieldOffset": 16, "Name": "R21"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7263, "FieldOffset": 32, "Name": "R22"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7263, "FieldOffset": 48, "Name": "R23"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7263, "FieldOffset": 64, "Name": "R24"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7263, "FieldOffset": 80, "Name": "S21"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7263, "FieldOffset": 96, "Name": "S22"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7263, "FieldOffset": 112, "Name": "S23"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7263, "FieldOffset": 128, "Name": "S24"}}, {"Kind": "LF_NESTTYPE", "NestedType": {"Type": 7263, "Name": ""}}]}
// [TPI 0x1c61] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 10, "Options": ["None", "ContainsNestedClass"], "FieldList": 7264, "Name": "poly1305_power_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 144}}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: poly1305_vec.c.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===
