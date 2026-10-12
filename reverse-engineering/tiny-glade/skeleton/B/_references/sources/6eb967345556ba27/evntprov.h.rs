// Generated from PDB metadata by skeleton/_tools/fill_type_structures.py.
// Version: B
// 类型元数据；原作者源码文本/定义文件见逐项映射；不生成依赖实现。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: evntprov.h.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB forward_declaration_only] _EVENT_DATA_DESCRIPTOR; TPI 0x2307
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "_EVENT_DATA_DESCRIPTOR", "UniqueName": ".?AU_EVENT_DATA_DESCRIPTOR@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [8970]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] _EVENT_DATA_DESCRIPTOR; TPI 0x230a
// 原始定义文件映射: PDB_UDT_source_line
// size = 16; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 6, "Options": ["None", "HasUniqueName"], "FieldList": 8969, "Name": "_EVENT_DATA_DESCRIPTOR", "UniqueName": ".?AU_EVENT_DATA_DESCRIPTOR@@", "DerivationList": 0, "VTableShape": 0, "Size": 16}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8969, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 0, "Name": "Ptr"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8969, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 8, "Name": "Size"}}, "type_references": [{"index": 34, "spelling": "UInt32Long", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8969, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 12, "Name": "Reserved"}}, "type_references": [{"index": 34, "spelling": "UInt32Long", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8969, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 32, "FieldOffset": 12, "Name": "Type"}}, "type_references": [{"index": 32, "spelling": "UnsignedCharacter", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8969, "order": 4, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 32, "FieldOffset": 13, "Name": "Reserved1"}}, "type_references": [{"index": 32, "spelling": "UnsignedCharacter", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8969, "order": 5, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 33, "FieldOffset": 14, "Name": "Reserved2"}}, "type_references": [{"index": 33, "spelling": "UInt16Short", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 134125, "original_path": "d:\\a01\\_work\\5\\s\\src\\ExternalAPIs\\Windows\\10\\sdk\\inc\\evntprov.h", "line": 178, "module": null, "source_string_id": 134124}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] _EVENT_DESCRIPTOR; TPI 0x230b
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "_EVENT_DESCRIPTOR", "UniqueName": ".?AU_EVENT_DESCRIPTOR@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [8974]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] _EVENT_DESCRIPTOR; TPI 0x230e
// 原始定义文件映射: PDB_UDT_source_line
// size = 16; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 7, "Options": ["None", "HasUniqueName"], "FieldList": 8973, "Name": "_EVENT_DESCRIPTOR", "UniqueName": ".?AU_EVENT_DESCRIPTOR@@", "DerivationList": 0, "VTableShape": 0, "Size": 16}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8973, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 33, "FieldOffset": 0, "Name": "Id"}}, "type_references": [{"index": 33, "spelling": "UInt16Short", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8973, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 32, "FieldOffset": 2, "Name": "Version"}}, "type_references": [{"index": 32, "spelling": "UnsignedCharacter", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8973, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 32, "FieldOffset": 3, "Name": "Channel"}}, "type_references": [{"index": 32, "spelling": "UnsignedCharacter", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8973, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 32, "FieldOffset": 4, "Name": "Level"}}, "type_references": [{"index": 32, "spelling": "UnsignedCharacter", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8973, "order": 4, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 32, "FieldOffset": 5, "Name": "Opcode"}}, "type_references": [{"index": 32, "spelling": "UnsignedCharacter", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8973, "order": 5, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 33, "FieldOffset": 6, "Name": "Task"}}, "type_references": [{"index": 33, "spelling": "UInt16Short", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8973, "order": 6, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 8, "Name": "Keyword"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 134126, "original_path": "d:\\a01\\_work\\5\\s\\src\\ExternalAPIs\\Windows\\10\\sdk\\inc\\evntprov.h", "line": 204, "module": null, "source_string_id": 134124}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] _EVENT_DATA_DESCRIPTOR::<unnamed-tag>; TPI 0x2313
// 原始定义文件映射: PDB_UDT_source_line
// size = 4; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_UNION", "Union": {"MemberCount": 4, "Options": ["None", "Nested", "HasUniqueName", "Sealed"], "FieldList": 8978, "Name": "_EVENT_DATA_DESCRIPTOR::<unnamed-tag>", "UniqueName": ".?AT<unnamed-tag>@_EVENT_DATA_DESCRIPTOR@@", "Size": 4}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8978, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 0, "Name": "Reserved"}}, "type_references": [{"index": 34, "spelling": "UInt32Long", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8978, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 32, "FieldOffset": 0, "Name": "Type"}}, "type_references": [{"index": 32, "spelling": "UnsignedCharacter", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8978, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 32, "FieldOffset": 1, "Name": "Reserved1"}}, "type_references": [{"index": 32, "spelling": "UnsignedCharacter", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8978, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 33, "FieldOffset": 2, "Name": "Reserved2"}}, "type_references": [{"index": 33, "spelling": "UInt16Short", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 134129, "original_path": "d:\\a01\\_work\\5\\s\\src\\ExternalAPIs\\Windows\\10\\sdk\\inc\\evntprov.h", "line": 182, "module": null, "source_string_id": 134124}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] _EVENT_DATA_DESCRIPTOR::<unnamed-tag>::<unnamed-tag>; TPI 0x2315
// 原始定义文件映射: PDB_UDT_source_line
// size = 4; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "Nested", "HasUniqueName"], "FieldList": 8980, "Name": "_EVENT_DATA_DESCRIPTOR::<unnamed-tag>::<unnamed-tag>", "UniqueName": ".?AU<unnamed-tag>@0_EVENT_DATA_DESCRIPTOR@@", "DerivationList": 0, "VTableShape": 0, "Size": 4}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8980, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 32, "FieldOffset": 0, "Name": "Type"}}, "type_references": [{"index": 32, "spelling": "UnsignedCharacter", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8980, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 32, "FieldOffset": 1, "Name": "Reserved1"}}, "type_references": [{"index": 32, "spelling": "UnsignedCharacter", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8980, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 33, "FieldOffset": 2, "Name": "Reserved2"}}, "type_references": [{"index": 33, "spelling": "UInt16Short", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 134130, "original_path": "d:\\a01\\_work\\5\\s\\src\\ExternalAPIs\\Windows\\10\\sdk\\inc\\evntprov.h", "line": 184, "module": null, "source_string_id": 134124}
// === 完整 TPI 结构关系引用记录；原文不转换成猜测的 Rust ABI ===
// [TPI 0x2307] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "_EVENT_DATA_DESCRIPTOR", "UniqueName": ".?AU_EVENT_DATA_DESCRIPTOR@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x2309] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 0, "Name": "Ptr"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 8, "Name": "Size"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 12, "Name": "Reserved"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 32, "FieldOffset": 12, "Name": "Type"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 32, "FieldOffset": 13, "Name": "Reserved1"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 33, "FieldOffset": 14, "Name": "Reserved2"}}]}
// [TPI 0x230a] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 6, "Options": ["None", "HasUniqueName"], "FieldList": 8969, "Name": "_EVENT_DATA_DESCRIPTOR", "UniqueName": ".?AU_EVENT_DATA_DESCRIPTOR@@", "DerivationList": 0, "VTableShape": 0, "Size": 16}}
// [TPI 0x230b] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "_EVENT_DESCRIPTOR", "UniqueName": ".?AU_EVENT_DESCRIPTOR@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x230d] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 33, "FieldOffset": 0, "Name": "Id"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 32, "FieldOffset": 2, "Name": "Version"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 32, "FieldOffset": 3, "Name": "Channel"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 32, "FieldOffset": 4, "Name": "Level"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 32, "FieldOffset": 5, "Name": "Opcode"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 33, "FieldOffset": 6, "Name": "Task"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 8, "Name": "Keyword"}}]}
// [TPI 0x230e] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 7, "Options": ["None", "HasUniqueName"], "FieldList": 8973, "Name": "_EVENT_DESCRIPTOR", "UniqueName": ".?AU_EVENT_DESCRIPTOR@@", "DerivationList": 0, "VTableShape": 0, "Size": 16}}
// [TPI 0x2312] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 0, "Name": "Reserved"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 32, "FieldOffset": 0, "Name": "Type"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 32, "FieldOffset": 1, "Name": "Reserved1"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 33, "FieldOffset": 2, "Name": "Reserved2"}}]}
// [TPI 0x2313] {"Kind": "LF_UNION", "Union": {"MemberCount": 4, "Options": ["None", "Nested", "HasUniqueName", "Sealed"], "FieldList": 8978, "Name": "_EVENT_DATA_DESCRIPTOR::<unnamed-tag>", "UniqueName": ".?AT<unnamed-tag>@_EVENT_DATA_DESCRIPTOR@@", "Size": 4}}
// [TPI 0x2314] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 32, "FieldOffset": 0, "Name": "Type"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 32, "FieldOffset": 1, "Name": "Reserved1"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 33, "FieldOffset": 2, "Name": "Reserved2"}}]}
// [TPI 0x2315] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "Nested", "HasUniqueName"], "FieldList": 8980, "Name": "_EVENT_DATA_DESCRIPTOR::<unnamed-tag>::<unnamed-tag>", "UniqueName": ".?AU<unnamed-tag>@0_EVENT_DATA_DESCRIPTOR@@", "DerivationList": 0, "VTableShape": 0, "Size": 4}}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: evntprov.h.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===
