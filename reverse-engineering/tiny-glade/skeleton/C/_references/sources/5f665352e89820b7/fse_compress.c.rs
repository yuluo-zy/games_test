// Generated from PDB metadata by skeleton/_tools/fill_type_structures.py.
// Version: C
// 类型元数据；原作者源码文本/定义文件见逐项映射；不生成依赖实现。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: fse_compress.c.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB forward_declaration_only] fseWkspMax_t; TPI 0x1a65
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "fseWkspMax_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [6763]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] fseWkspMax_t::<unnamed-tag>; TPI 0x1a69
// 原始定义文件映射: PDB_UDT_source_line
// size = 4096; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_UNION", "Union": {"MemberCount": 2, "Options": ["None", "Nested", "Sealed"], "FieldList": 6760, "Name": "fseWkspMax_t::<unnamed-tag>", "UniqueName": "", "Size": 4096}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6760, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6543, "FieldOffset": 0, "Name": "hist_wksp"}}, "type_references": [{"index": 6543, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 117, "IndexType": 35, "Size": 4096, "Name": ""}}, "nested_types": {"ElementType": {"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6760, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6759, "FieldOffset": 0, "Name": "scratchBuffer"}}, "type_references": [{"index": 6759, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 32, "IndexType": 35, "Size": 4096, "Name": ""}}, "nested_types": {"ElementType": {"index": 32, "spelling": "UnsignedCharacter", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [PDB UDT 源码行] {"ipi_index": 131423, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/fse_compress.c", "line": 721, "module": null, "source_string_id": 131422}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] fseWkspMax_t; TPI 0x1a6b
// 原始定义文件映射: PDB_UDT_source_line
// size = 14340; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "ContainsNestedClass"], "FieldList": 6762, "Name": "fseWkspMax_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 14340}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6762, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6758, "FieldOffset": 0, "Name": "CTable_max"}}, "type_references": [{"index": 6758, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 117, "IndexType": 35, "Size": 10244, "Name": ""}}, "nested_types": {"ElementType": {"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6762, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6761, "FieldOffset": 10244, "Name": "workspace"}}, "type_references": [{"index": 6761, "kind": "LF_UNION", "spelling": "fseWkspMax_t::<unnamed-tag>", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6762, "order": 2, "kind": "LF_NESTTYPE", "record": {"Kind": "LF_NESTTYPE", "NestedType": {"Type": 6761, "Name": ""}}, "type_references": [{"index": 6761, "kind": "LF_UNION", "spelling": "fseWkspMax_t::<unnamed-tag>", "status": "PDB_record_name"}]}
// [PDB UDT 源码行] {"ipi_index": 131424, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/fse_compress.c", "line": 719, "module": null, "source_string_id": 131422}
// === 完整 TPI 结构关系引用记录；原文不转换成猜测的 Rust ABI ===
// [TPI 0x198f] {"Kind": "LF_ARRAY", "Array": {"ElementType": 117, "IndexType": 35, "Size": 4096, "Name": ""}}
// [TPI 0x1a65] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "fseWkspMax_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1a66] {"Kind": "LF_ARRAY", "Array": {"ElementType": 117, "IndexType": 35, "Size": 10244, "Name": ""}}
// [TPI 0x1a67] {"Kind": "LF_ARRAY", "Array": {"ElementType": 32, "IndexType": 35, "Size": 4096, "Name": ""}}
// [TPI 0x1a68] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6543, "FieldOffset": 0, "Name": "hist_wksp"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6759, "FieldOffset": 0, "Name": "scratchBuffer"}}]}
// [TPI 0x1a69] {"Kind": "LF_UNION", "Union": {"MemberCount": 2, "Options": ["None", "Nested", "Sealed"], "FieldList": 6760, "Name": "fseWkspMax_t::<unnamed-tag>", "UniqueName": "", "Size": 4096}}
// [TPI 0x1a6a] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6758, "FieldOffset": 0, "Name": "CTable_max"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6761, "FieldOffset": 10244, "Name": "workspace"}}, {"Kind": "LF_NESTTYPE", "NestedType": {"Type": 6761, "Name": ""}}]}
// [TPI 0x1a6b] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "ContainsNestedClass"], "FieldList": 6762, "Name": "fseWkspMax_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 14340}}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: fse_compress.c.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===
