// Generated from PDB metadata by skeleton/_tools/fill_type_structures.py.
// Version: A
// 类型元数据；原作者源码文本/定义文件见逐项映射；不生成依赖实现。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: ehdata.h.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB forward_declaration_only] EHExceptionRecord; TPI 0x27c5
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "EHExceptionRecord", "UniqueName": ".?AUEHExceptionRecord@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [10185]
// ------------------------------------------------------------------
// [PDB forward_declaration_only] EHExceptionRecord::EHParameters; TPI 0x27c7
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "Nested", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "EHExceptionRecord::EHParameters", "UniqueName": ".?AUEHParameters@EHExceptionRecord@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [10193]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] EHExceptionRecord; TPI 0x27c9
// 原始定义文件映射: PDB_UDT_source_line
// size = 64; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 7, "Options": ["None", "ContainsNestedClass", "HasUniqueName"], "FieldList": 10184, "Name": "EHExceptionRecord", "UniqueName": ".?AUEHExceptionRecord@@", "DerivationList": 0, "VTableShape": 0, "Size": 64}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 10184, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 0, "Name": "ExceptionCode"}}, "type_references": [{"index": 34, "spelling": "UInt32Long", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 10184, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 4, "Name": "ExceptionFlags"}}, "type_references": [{"index": 34, "spelling": "UInt32Long", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 10184, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7464, "FieldOffset": 8, "Name": "ExceptionRecord"}}, "type_references": [{"index": 7464, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 7463, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 7463, "kind": "LF_STRUCTURE", "spelling": "_EXCEPTION_RECORD", "status": "PDB_record_name"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 10184, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 16, "Name": "ExceptionAddress"}}, "type_references": [{"index": 1539, "spelling": "Void / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 10184, "order": 4, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 24, "Name": "NumberParameters"}}, "type_references": [{"index": 34, "spelling": "UInt32Long", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 10184, "order": 5, "kind": "LF_NESTTYPE", "record": {"Kind": "LF_NESTTYPE", "NestedType": {"Type": 10183, "Name": "EHParameters"}}, "type_references": [{"index": 10183, "kind": "LF_STRUCTURE", "spelling": "EHExceptionRecord::EHParameters", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 10184, "order": 6, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 10183, "FieldOffset": 32, "Name": "params"}}, "type_references": [{"index": 10183, "kind": "LF_STRUCTURE", "spelling": "EHExceptionRecord::EHParameters", "status": "PDB_record_name"}]}
// [PDB UDT 源码行] {"ipi_index": 123630, "original_path": "d:\\a01\\_work\\5\\s\\src\\vctools\\crt\\vcruntime\\inc\\ehdata.h", "line": 509, "module": null, "source_string_id": 123629}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] EHExceptionRecord::EHParameters; TPI 0x27d1
// 原始定义文件映射: PDB_UDT_source_line
// size = 32; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None", "Nested", "HasUniqueName"], "FieldList": 10192, "Name": "EHExceptionRecord::EHParameters", "UniqueName": ".?AUEHParameters@EHExceptionRecord@@", "DerivationList": 0, "VTableShape": 0, "Size": 32}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 10192, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 0, "Name": "magicNumber"}}, "type_references": [{"index": 34, "spelling": "UInt32Long", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 10192, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "pExceptionObject"}}, "type_references": [{"index": 1539, "spelling": "Void / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 10192, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 10191, "FieldOffset": 16, "Name": "pThrowInfo"}}, "type_references": [{"index": 10191, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 10190, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 10190, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 10189, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 10189, "kind": "LF_STRUCTURE", "spelling": "_s_ThrowInfo", "status": "PDB_record_name"}}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 10192, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 24, "Name": "pThrowImageBase"}}, "type_references": [{"index": 1539, "spelling": "Void / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 123631, "original_path": "d:\\a01\\_work\\5\\s\\src\\vctools\\crt\\vcruntime\\inc\\ehdata.h", "line": 515, "module": null, "source_string_id": 123629}
// === 完整 TPI 结构关系引用记录；原文不转换成猜测的 Rust ABI ===
// [TPI 0x1d27] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "_EXCEPTION_RECORD", "UniqueName": ".?AU_EXCEPTION_RECORD@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1d28] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 7463, "Attrs": 65548}}
// [TPI 0x27c5] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "EHExceptionRecord", "UniqueName": ".?AUEHExceptionRecord@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x27c7] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "Nested", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "EHExceptionRecord::EHParameters", "UniqueName": ".?AUEHParameters@EHExceptionRecord@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x27c8] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 0, "Name": "ExceptionCode"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 4, "Name": "ExceptionFlags"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7464, "FieldOffset": 8, "Name": "ExceptionRecord"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 16, "Name": "ExceptionAddress"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 24, "Name": "NumberParameters"}}, {"Kind": "LF_NESTTYPE", "NestedType": {"Type": 10183, "Name": "EHParameters"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 10183, "FieldOffset": 32, "Name": "params"}}]}
// [TPI 0x27c9] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 7, "Options": ["None", "ContainsNestedClass", "HasUniqueName"], "FieldList": 10184, "Name": "EHExceptionRecord", "UniqueName": ".?AUEHExceptionRecord@@", "DerivationList": 0, "VTableShape": 0, "Size": 64}}
// [TPI 0x27cd] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "_s_ThrowInfo", "UniqueName": ".?AU_s_ThrowInfo@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x27ce] {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 10189, "Modifiers": ["None", "Const"]}}
// [TPI 0x27cf] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 10190, "Attrs": 65548}}
// [TPI 0x27d0] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 0, "Name": "magicNumber"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "pExceptionObject"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 10191, "FieldOffset": 16, "Name": "pThrowInfo"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 24, "Name": "pThrowImageBase"}}]}
// [TPI 0x27d1] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None", "Nested", "HasUniqueName"], "FieldList": 10192, "Name": "EHExceptionRecord::EHParameters", "UniqueName": ".?AUEHParameters@EHExceptionRecord@@", "DerivationList": 0, "VTableShape": 0, "Size": 32}}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: ehdata.h.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===
