// Generated from PDB metadata by skeleton/_tools/fill_type_structures.py.
// Version: C
// 类型元数据；原作者源码文本/定义文件见逐项映射；不生成依赖实现。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: poly1305_vec.c.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB forward_declaration_only] poly1305_state_internal_t; TPI 0x1f09
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "poly1305_state_internal_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [7956]
// ------------------------------------------------------------------
// [PDB forward_declaration_only] poly1305_power_t; TPI 0x1f0e
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "poly1305_power_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [7960]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] poly1305_state_internal_t; TPI 0x1f14
// 原始定义文件映射: PDB_UDT_source_line
// size = 448; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 6, "Options": ["None"], "FieldList": 7955, "Name": "poly1305_state_internal_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 448}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7955, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7951, "FieldOffset": 0, "Name": "P"}}, "type_references": [{"index": 7951, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 7950, "IndexType": 35, "Size": 288, "Name": ""}}, "nested_types": {"ElementType": {"index": 7950, "kind": "LF_STRUCTURE", "spelling": "poly1305_power_t", "status": "PDB_record_name"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7955, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7952, "FieldOffset": 288, "Name": "H"}}, "type_references": [{"index": 7952, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 5423, "IndexType": 35, "Size": 80, "Name": ""}}, "nested_types": {"ElementType": {"index": 5423, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 19, "IndexType": 35, "Size": 16, "Name": ""}}, "nested_types": {"ElementType": {"index": 19, "spelling": "Int64Quad", "status": "CodeView_simple_type; not Rust source spelling"}}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7955, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7953, "FieldOffset": 288, "Name": "HH"}}, "type_references": [{"index": 7953, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 35, "IndexType": 35, "Size": 80, "Name": ""}}, "nested_types": {"ElementType": {"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7955, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 368, "Name": "started"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7955, "order": 4, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 376, "Name": "leftover"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7955, "order": 5, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7954, "FieldOffset": 384, "Name": "buffer"}}, "type_references": [{"index": 7954, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 32, "IndexType": 35, "Size": 64, "Name": ""}}, "nested_types": {"ElementType": {"index": 32, "spelling": "UnsignedCharacter", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [PDB UDT 源码行] {"ipi_index": 138649, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ring-0.16.20/crypto/poly1305/poly1305_vec.c", "line": 84, "module": null, "source_string_id": 138648}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] poly1305_power_t::<unnamed-tag>; TPI 0x1f16
// 原始定义文件映射: PDB_UDT_source_line
// size = 16; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_UNION", "Union": {"MemberCount": 3, "Options": ["None", "Nested", "Sealed"], "FieldList": 7957, "Name": "poly1305_power_t::<unnamed-tag>", "UniqueName": "", "Size": 16}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7957, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5423, "FieldOffset": 0, "Name": "v"}}, "type_references": [{"index": 5423, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 19, "IndexType": 35, "Size": 16, "Name": ""}}, "nested_types": {"ElementType": {"index": 19, "spelling": "Int64Quad", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7957, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6578, "FieldOffset": 0, "Name": "u"}}, "type_references": [{"index": 6578, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 35, "IndexType": 35, "Size": 16, "Name": ""}}, "nested_types": {"ElementType": {"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7957, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4240, "FieldOffset": 0, "Name": "d"}}, "type_references": [{"index": 4240, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 117, "IndexType": 35, "Size": 16, "Name": ""}}, "nested_types": {"ElementType": {"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [PDB UDT 源码行] {"ipi_index": 138650, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ring-0.16.20/crypto/poly1305/poly1305_vec.c", "line": 77, "module": null, "source_string_id": 138648}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] poly1305_power_t; TPI 0x1f18
// 原始定义文件映射: PDB_UDT_source_line
// size = 144; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 10, "Options": ["None", "ContainsNestedClass"], "FieldList": 7959, "Name": "poly1305_power_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 144}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7959, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7958, "FieldOffset": 0, "Name": "R20"}}, "type_references": [{"index": 7958, "kind": "LF_UNION", "spelling": "poly1305_power_t::<unnamed-tag>", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7959, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7958, "FieldOffset": 16, "Name": "R21"}}, "type_references": [{"index": 7958, "kind": "LF_UNION", "spelling": "poly1305_power_t::<unnamed-tag>", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7959, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7958, "FieldOffset": 32, "Name": "R22"}}, "type_references": [{"index": 7958, "kind": "LF_UNION", "spelling": "poly1305_power_t::<unnamed-tag>", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7959, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7958, "FieldOffset": 48, "Name": "R23"}}, "type_references": [{"index": 7958, "kind": "LF_UNION", "spelling": "poly1305_power_t::<unnamed-tag>", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7959, "order": 4, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7958, "FieldOffset": 64, "Name": "R24"}}, "type_references": [{"index": 7958, "kind": "LF_UNION", "spelling": "poly1305_power_t::<unnamed-tag>", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7959, "order": 5, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7958, "FieldOffset": 80, "Name": "S21"}}, "type_references": [{"index": 7958, "kind": "LF_UNION", "spelling": "poly1305_power_t::<unnamed-tag>", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7959, "order": 6, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7958, "FieldOffset": 96, "Name": "S22"}}, "type_references": [{"index": 7958, "kind": "LF_UNION", "spelling": "poly1305_power_t::<unnamed-tag>", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7959, "order": 7, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7958, "FieldOffset": 112, "Name": "S23"}}, "type_references": [{"index": 7958, "kind": "LF_UNION", "spelling": "poly1305_power_t::<unnamed-tag>", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7959, "order": 8, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7958, "FieldOffset": 128, "Name": "S24"}}, "type_references": [{"index": 7958, "kind": "LF_UNION", "spelling": "poly1305_power_t::<unnamed-tag>", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7959, "order": 9, "kind": "LF_NESTTYPE", "record": {"Kind": "LF_NESTTYPE", "NestedType": {"Type": 7958, "Name": ""}}, "type_references": [{"index": 7958, "kind": "LF_UNION", "spelling": "poly1305_power_t::<unnamed-tag>", "status": "PDB_record_name"}]}
// [PDB UDT 源码行] {"ipi_index": 138651, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ring-0.16.20/crypto/poly1305/poly1305_vec.c", "line": 76, "module": null, "source_string_id": 138648}
// === 完整 TPI 结构关系引用记录；原文不转换成猜测的 Rust ABI ===
// [TPI 0x1090] {"Kind": "LF_ARRAY", "Array": {"ElementType": 117, "IndexType": 35, "Size": 16, "Name": ""}}
// [TPI 0x152f] {"Kind": "LF_ARRAY", "Array": {"ElementType": 19, "IndexType": 35, "Size": 16, "Name": ""}}
// [TPI 0x19b2] {"Kind": "LF_ARRAY", "Array": {"ElementType": 35, "IndexType": 35, "Size": 16, "Name": ""}}
// [TPI 0x1f09] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "poly1305_state_internal_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1f0e] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "poly1305_power_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1f0f] {"Kind": "LF_ARRAY", "Array": {"ElementType": 7950, "IndexType": 35, "Size": 288, "Name": ""}}
// [TPI 0x1f10] {"Kind": "LF_ARRAY", "Array": {"ElementType": 5423, "IndexType": 35, "Size": 80, "Name": ""}}
// [TPI 0x1f11] {"Kind": "LF_ARRAY", "Array": {"ElementType": 35, "IndexType": 35, "Size": 80, "Name": ""}}
// [TPI 0x1f12] {"Kind": "LF_ARRAY", "Array": {"ElementType": 32, "IndexType": 35, "Size": 64, "Name": ""}}
// [TPI 0x1f13] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7951, "FieldOffset": 0, "Name": "P"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7952, "FieldOffset": 288, "Name": "H"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7953, "FieldOffset": 288, "Name": "HH"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 368, "Name": "started"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 376, "Name": "leftover"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7954, "FieldOffset": 384, "Name": "buffer"}}]}
// [TPI 0x1f14] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 6, "Options": ["None"], "FieldList": 7955, "Name": "poly1305_state_internal_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 448}}
// [TPI 0x1f15] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5423, "FieldOffset": 0, "Name": "v"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6578, "FieldOffset": 0, "Name": "u"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4240, "FieldOffset": 0, "Name": "d"}}]}
// [TPI 0x1f16] {"Kind": "LF_UNION", "Union": {"MemberCount": 3, "Options": ["None", "Nested", "Sealed"], "FieldList": 7957, "Name": "poly1305_power_t::<unnamed-tag>", "UniqueName": "", "Size": 16}}
// [TPI 0x1f17] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7958, "FieldOffset": 0, "Name": "R20"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7958, "FieldOffset": 16, "Name": "R21"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7958, "FieldOffset": 32, "Name": "R22"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7958, "FieldOffset": 48, "Name": "R23"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7958, "FieldOffset": 64, "Name": "R24"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7958, "FieldOffset": 80, "Name": "S21"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7958, "FieldOffset": 96, "Name": "S22"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7958, "FieldOffset": 112, "Name": "S23"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7958, "FieldOffset": 128, "Name": "S24"}}, {"Kind": "LF_NESTTYPE", "NestedType": {"Type": 7958, "Name": ""}}]}
// [TPI 0x1f18] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 10, "Options": ["None", "ContainsNestedClass"], "FieldList": 7959, "Name": "poly1305_power_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 144}}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: poly1305_vec.c.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===
