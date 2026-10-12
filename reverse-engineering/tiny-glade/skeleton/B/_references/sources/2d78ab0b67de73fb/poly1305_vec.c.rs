// Generated from PDB metadata by skeleton/_tools/fill_type_structures.py.
// Version: B
// 类型元数据；原作者源码文本/定义文件见逐项映射；不生成依赖实现。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: poly1305_vec.c.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB forward_declaration_only] poly1305_state_internal_t; TPI 0x1a85
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "poly1305_state_internal_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [6801]
// ------------------------------------------------------------------
// [PDB forward_declaration_only] poly1305_power_t; TPI 0x1a8b
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "poly1305_power_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [6806]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] poly1305_state_internal_t; TPI 0x1a91
// 原始定义文件映射: PDB_UDT_source_line
// size = 448; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 6, "Options": ["None"], "FieldList": 6800, "Name": "poly1305_state_internal_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 448}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6800, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6796, "FieldOffset": 0, "Name": "P"}}, "type_references": [{"index": 6796, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 6795, "IndexType": 35, "Size": 288, "Name": ""}}, "nested_types": {"ElementType": {"index": 6795, "kind": "LF_STRUCTURE", "spelling": "poly1305_power_t", "status": "PDB_record_name"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6800, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6797, "FieldOffset": 288, "Name": "H"}}, "type_references": [{"index": 6797, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 6293, "IndexType": 35, "Size": 80, "Name": ""}}, "nested_types": {"ElementType": {"index": 6293, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 19, "IndexType": 35, "Size": 16, "Name": ""}}, "nested_types": {"ElementType": {"index": 19, "spelling": "Int64Quad", "status": "CodeView_simple_type; not Rust source spelling"}}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6800, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6798, "FieldOffset": 288, "Name": "HH"}}, "type_references": [{"index": 6798, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 35, "IndexType": 35, "Size": 80, "Name": ""}}, "nested_types": {"ElementType": {"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6800, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 368, "Name": "started"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6800, "order": 4, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 376, "Name": "leftover"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6800, "order": 5, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6799, "FieldOffset": 384, "Name": "buffer"}}, "type_references": [{"index": 6799, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 32, "IndexType": 35, "Size": 64, "Name": ""}}, "nested_types": {"ElementType": {"index": 32, "spelling": "UnsignedCharacter", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [PDB UDT 源码行] {"ipi_index": 124673, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ring-0.16.20/crypto/poly1305/poly1305_vec.c", "line": 84, "module": null, "source_string_id": 124672}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] poly1305_power_t::<unnamed-tag>; TPI 0x1a94
// 原始定义文件映射: PDB_UDT_source_line
// size = 16; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_UNION", "Union": {"MemberCount": 3, "Options": ["None", "Nested", "Sealed"], "FieldList": 6803, "Name": "poly1305_power_t::<unnamed-tag>", "UniqueName": "", "Size": 16}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6803, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6293, "FieldOffset": 0, "Name": "v"}}, "type_references": [{"index": 6293, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 19, "IndexType": 35, "Size": 16, "Name": ""}}, "nested_types": {"ElementType": {"index": 19, "spelling": "Int64Quad", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6803, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6802, "FieldOffset": 0, "Name": "u"}}, "type_references": [{"index": 6802, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 35, "IndexType": 35, "Size": 16, "Name": ""}}, "nested_types": {"ElementType": {"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6803, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4240, "FieldOffset": 0, "Name": "d"}}, "type_references": [{"index": 4240, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 117, "IndexType": 35, "Size": 16, "Name": ""}}, "nested_types": {"ElementType": {"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [PDB UDT 源码行] {"ipi_index": 124674, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ring-0.16.20/crypto/poly1305/poly1305_vec.c", "line": 77, "module": null, "source_string_id": 124672}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] poly1305_power_t; TPI 0x1a96
// 原始定义文件映射: PDB_UDT_source_line
// size = 144; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 10, "Options": ["None", "ContainsNestedClass"], "FieldList": 6805, "Name": "poly1305_power_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 144}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6805, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6804, "FieldOffset": 0, "Name": "R20"}}, "type_references": [{"index": 6804, "kind": "LF_UNION", "spelling": "poly1305_power_t::<unnamed-tag>", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6805, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6804, "FieldOffset": 16, "Name": "R21"}}, "type_references": [{"index": 6804, "kind": "LF_UNION", "spelling": "poly1305_power_t::<unnamed-tag>", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6805, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6804, "FieldOffset": 32, "Name": "R22"}}, "type_references": [{"index": 6804, "kind": "LF_UNION", "spelling": "poly1305_power_t::<unnamed-tag>", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6805, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6804, "FieldOffset": 48, "Name": "R23"}}, "type_references": [{"index": 6804, "kind": "LF_UNION", "spelling": "poly1305_power_t::<unnamed-tag>", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6805, "order": 4, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6804, "FieldOffset": 64, "Name": "R24"}}, "type_references": [{"index": 6804, "kind": "LF_UNION", "spelling": "poly1305_power_t::<unnamed-tag>", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6805, "order": 5, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6804, "FieldOffset": 80, "Name": "S21"}}, "type_references": [{"index": 6804, "kind": "LF_UNION", "spelling": "poly1305_power_t::<unnamed-tag>", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6805, "order": 6, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6804, "FieldOffset": 96, "Name": "S22"}}, "type_references": [{"index": 6804, "kind": "LF_UNION", "spelling": "poly1305_power_t::<unnamed-tag>", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6805, "order": 7, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6804, "FieldOffset": 112, "Name": "S23"}}, "type_references": [{"index": 6804, "kind": "LF_UNION", "spelling": "poly1305_power_t::<unnamed-tag>", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6805, "order": 8, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6804, "FieldOffset": 128, "Name": "S24"}}, "type_references": [{"index": 6804, "kind": "LF_UNION", "spelling": "poly1305_power_t::<unnamed-tag>", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6805, "order": 9, "kind": "LF_NESTTYPE", "record": {"Kind": "LF_NESTTYPE", "NestedType": {"Type": 6804, "Name": ""}}, "type_references": [{"index": 6804, "kind": "LF_UNION", "spelling": "poly1305_power_t::<unnamed-tag>", "status": "PDB_record_name"}]}
// [PDB UDT 源码行] {"ipi_index": 124675, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ring-0.16.20/crypto/poly1305/poly1305_vec.c", "line": 76, "module": null, "source_string_id": 124672}
// === 完整 TPI 结构关系引用记录；原文不转换成猜测的 Rust ABI ===
// [TPI 0x1090] {"Kind": "LF_ARRAY", "Array": {"ElementType": 117, "IndexType": 35, "Size": 16, "Name": ""}}
// [TPI 0x1895] {"Kind": "LF_ARRAY", "Array": {"ElementType": 19, "IndexType": 35, "Size": 16, "Name": ""}}
// [TPI 0x1a85] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "poly1305_state_internal_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1a8b] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "poly1305_power_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1a8c] {"Kind": "LF_ARRAY", "Array": {"ElementType": 6795, "IndexType": 35, "Size": 288, "Name": ""}}
// [TPI 0x1a8d] {"Kind": "LF_ARRAY", "Array": {"ElementType": 6293, "IndexType": 35, "Size": 80, "Name": ""}}
// [TPI 0x1a8e] {"Kind": "LF_ARRAY", "Array": {"ElementType": 35, "IndexType": 35, "Size": 80, "Name": ""}}
// [TPI 0x1a8f] {"Kind": "LF_ARRAY", "Array": {"ElementType": 32, "IndexType": 35, "Size": 64, "Name": ""}}
// [TPI 0x1a90] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6796, "FieldOffset": 0, "Name": "P"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6797, "FieldOffset": 288, "Name": "H"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6798, "FieldOffset": 288, "Name": "HH"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 368, "Name": "started"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 376, "Name": "leftover"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6799, "FieldOffset": 384, "Name": "buffer"}}]}
// [TPI 0x1a91] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 6, "Options": ["None"], "FieldList": 6800, "Name": "poly1305_state_internal_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 448}}
// [TPI 0x1a92] {"Kind": "LF_ARRAY", "Array": {"ElementType": 35, "IndexType": 35, "Size": 16, "Name": ""}}
// [TPI 0x1a93] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6293, "FieldOffset": 0, "Name": "v"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6802, "FieldOffset": 0, "Name": "u"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4240, "FieldOffset": 0, "Name": "d"}}]}
// [TPI 0x1a94] {"Kind": "LF_UNION", "Union": {"MemberCount": 3, "Options": ["None", "Nested", "Sealed"], "FieldList": 6803, "Name": "poly1305_power_t::<unnamed-tag>", "UniqueName": "", "Size": 16}}
// [TPI 0x1a95] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6804, "FieldOffset": 0, "Name": "R20"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6804, "FieldOffset": 16, "Name": "R21"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6804, "FieldOffset": 32, "Name": "R22"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6804, "FieldOffset": 48, "Name": "R23"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6804, "FieldOffset": 64, "Name": "R24"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6804, "FieldOffset": 80, "Name": "S21"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6804, "FieldOffset": 96, "Name": "S22"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6804, "FieldOffset": 112, "Name": "S23"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6804, "FieldOffset": 128, "Name": "S24"}}, {"Kind": "LF_NESTTYPE", "NestedType": {"Type": 6804, "Name": ""}}]}
// [TPI 0x1a96] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 10, "Options": ["None", "ContainsNestedClass"], "FieldList": 6805, "Name": "poly1305_power_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 144}}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: poly1305_vec.c.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===
