// Generated from PDB metadata by skeleton/_tools/fill_type_structures.py.
// Version: A
// 类型元数据；原作者源码文本/定义文件见逐项映射；不生成依赖实现。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: predefined C++ types (compiler internal).rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB forward_declaration_only] _s__RTTIBaseClassDescriptor; TPI 0x10eb
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "_s__RTTIBaseClassDescriptor", "UniqueName": ".?AU_s__RTTIBaseClassDescriptor@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [4340]
// ------------------------------------------------------------------
// [PDB forward_declaration_only] _TypeDescriptor; TPI 0x10ed
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "_TypeDescriptor", "UniqueName": ".?AU_TypeDescriptor@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [4395]
// ------------------------------------------------------------------
// [PDB forward_declaration_only] _PMD; TPI 0x10ef
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "_PMD", "UniqueName": ".?AU_PMD@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [4384]
// ------------------------------------------------------------------
// [PDB forward_declaration_only] _s__RTTIClassHierarchyDescriptor; TPI 0x10f0
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "_s__RTTIClassHierarchyDescriptor", "UniqueName": ".?AU_s__RTTIClassHierarchyDescriptor@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [4361]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] _s__RTTIBaseClassDescriptor; TPI 0x10f4
// 原始定义文件映射: PDB_UDT_source_line
// size = 36; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 5, "Options": ["None", "HasUniqueName"], "FieldList": 4339, "Name": "_s__RTTIBaseClassDescriptor", "UniqueName": ".?AU_s__RTTIBaseClassDescriptor@@", "DerivationList": 0, "VTableShape": 0, "Size": 36}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4339, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4334, "FieldOffset": 0, "Name": "pTypeDescriptor"}}, "type_references": [{"index": 4334, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4333, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4333, "kind": "LF_STRUCTURE", "spelling": "_TypeDescriptor", "status": "PDB_record_name"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4339, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 8, "Name": "numContainedBases"}}, "type_references": [{"index": 34, "spelling": "UInt32Long", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4339, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4335, "FieldOffset": 12, "Name": "where"}}, "type_references": [{"index": 4335, "kind": "LF_STRUCTURE", "spelling": "_PMD", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4339, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 24, "Name": "attributes"}}, "type_references": [{"index": 34, "spelling": "UInt32Long", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4339, "order": 4, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4338, "FieldOffset": 28, "Name": "pClassDescriptor"}}, "type_references": [{"index": 4338, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4337, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4337, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 4336, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 4336, "kind": "LF_STRUCTURE", "spelling": "_s__RTTIClassHierarchyDescriptor", "status": "PDB_record_name"}}}}}]}
// [PDB UDT 源码行] {"ipi_index": 44346, "original_path": "d:\\a01\\_work\\5\\s\\src\\vctools\\crt\\vcstartup\\build\\md\\msvcrt_kernel32\\predefined C++ types (compiler internal)", "line": 188, "module": null, "source_string_id": 44345}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] _s__RTTICompleteObjectLocator2; TPI 0x10f5
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "_s__RTTICompleteObjectLocator2", "UniqueName": ".?AU_s__RTTICompleteObjectLocator2@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [4345]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] _s__RTTICompleteObjectLocator2; TPI 0x10f9
// 原始定义文件映射: PDB_UDT_source_line
// size = 36; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 6, "Options": ["None", "HasUniqueName"], "FieldList": 4344, "Name": "_s__RTTICompleteObjectLocator2", "UniqueName": ".?AU_s__RTTICompleteObjectLocator2@@", "DerivationList": 0, "VTableShape": 0, "Size": 36}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4344, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 0, "Name": "signature"}}, "type_references": [{"index": 34, "spelling": "UInt32Long", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4344, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 4, "Name": "offset"}}, "type_references": [{"index": 34, "spelling": "UInt32Long", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4344, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 8, "Name": "cdOffset"}}, "type_references": [{"index": 34, "spelling": "UInt32Long", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4344, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4334, "FieldOffset": 12, "Name": "pTypeDescriptor"}}, "type_references": [{"index": 4334, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4333, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4333, "kind": "LF_STRUCTURE", "spelling": "_TypeDescriptor", "status": "PDB_record_name"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4344, "order": 4, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4338, "FieldOffset": 20, "Name": "pClassDescriptor"}}, "type_references": [{"index": 4338, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4337, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4337, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 4336, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 4336, "kind": "LF_STRUCTURE", "spelling": "_s__RTTIClassHierarchyDescriptor", "status": "PDB_record_name"}}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4344, "order": 5, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4343, "FieldOffset": 28, "Name": "pSelf"}}, "type_references": [{"index": 4343, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4342, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4342, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 4341, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 4341, "kind": "LF_STRUCTURE", "spelling": "_s__RTTICompleteObjectLocator2", "status": "PDB_record_name"}}}}}]}
// [PDB UDT 源码行] {"ipi_index": 44347, "original_path": "d:\\a01\\_work\\5\\s\\src\\vctools\\crt\\vcstartup\\build\\md\\msvcrt_kernel32\\predefined C++ types (compiler internal)", "line": 207, "module": null, "source_string_id": 44345}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] $_s__RTTIBaseClassArray$_extraBytes_24; TPI 0x10fa
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_s__RTTIBaseClassArray$_extraBytes_24", "UniqueName": ".?AU$_s__RTTIBaseClassArray$_extraBytes_24@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [4350]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] $_s__RTTIBaseClassArray$_extraBytes_24; TPI 0x10fe
// 原始定义文件映射: PDB_UDT_source_line
// size = 28; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 1, "Options": ["None", "HasUniqueName"], "FieldList": 4349, "Name": "$_s__RTTIBaseClassArray$_extraBytes_24", "UniqueName": ".?AU$_s__RTTIBaseClassArray$_extraBytes_24@@", "DerivationList": 0, "VTableShape": 0, "Size": 28}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4349, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4348, "FieldOffset": 0, "Name": "arrayOfBaseClassDescriptors"}}, "type_references": [{"index": 4348, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 4347, "IndexType": 35, "Size": 24, "Name": ""}}, "nested_types": {"ElementType": {"index": 4347, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4332, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4332, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 4331, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 4331, "kind": "LF_STRUCTURE", "spelling": "_s__RTTIBaseClassDescriptor", "status": "PDB_record_name"}}}}}}}]}
// [PDB UDT 源码行] {"ipi_index": 44348, "original_path": "d:\\a01\\_work\\5\\s\\src\\vctools\\crt\\vcstartup\\build\\md\\msvcrt_kernel32\\predefined C++ types (compiler internal)", "line": 197, "module": null, "source_string_id": 44345}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] $_TypeDescriptor$_extraBytes_20; TPI 0x10ff
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_TypeDescriptor$_extraBytes_20", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_20@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [4356]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] $_TypeDescriptor$_extraBytes_20; TPI 0x1104
// 原始定义文件映射: PDB_UDT_source_line
// size = 36; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "HasUniqueName"], "FieldList": 4355, "Name": "$_TypeDescriptor$_extraBytes_20", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_20@@", "DerivationList": 0, "VTableShape": 0, "Size": 36}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4355, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4353, "FieldOffset": 0, "Name": "pVFTable"}}, "type_references": [{"index": 4353, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4352, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4352, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 3, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 3, "spelling": "Void", "status": "CodeView_simple_type; not Rust source spelling"}}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4355, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "spare"}}, "type_references": [{"index": 1539, "spelling": "Void / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4355, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4354, "FieldOffset": 16, "Name": "name"}}, "type_references": [{"index": 4354, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 112, "IndexType": 35, "Size": 20, "Name": ""}}, "nested_types": {"ElementType": {"index": 112, "spelling": "NarrowCharacter", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [PDB UDT 源码行] {"ipi_index": 44349, "original_path": "d:\\a01\\_work\\5\\s\\src\\vctools\\crt\\vcstartup\\build\\md\\msvcrt_kernel32\\predefined C++ types (compiler internal)", "line": 134, "module": null, "source_string_id": 44345}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] _s__RTTIBaseClassArray; TPI 0x1105
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "_s__RTTIBaseClassArray", "UniqueName": ".?AU_s__RTTIBaseClassArray@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [4378]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] _s__RTTIClassHierarchyDescriptor; TPI 0x1109
// 原始定义文件映射: PDB_UDT_source_line
// size = 20; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None", "HasUniqueName"], "FieldList": 4360, "Name": "_s__RTTIClassHierarchyDescriptor", "UniqueName": ".?AU_s__RTTIClassHierarchyDescriptor@@", "DerivationList": 0, "VTableShape": 0, "Size": 20}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4360, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 0, "Name": "signature"}}, "type_references": [{"index": 34, "spelling": "UInt32Long", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4360, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 4, "Name": "attributes"}}, "type_references": [{"index": 34, "spelling": "UInt32Long", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4360, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 8, "Name": "numBaseClasses"}}, "type_references": [{"index": 34, "spelling": "UInt32Long", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4360, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4359, "FieldOffset": 12, "Name": "pBaseClassArray"}}, "type_references": [{"index": 4359, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4358, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4358, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 4357, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 4357, "kind": "LF_STRUCTURE", "spelling": "_s__RTTIBaseClassArray", "status": "PDB_record_name"}}}}}]}
// [PDB UDT 源码行] {"ipi_index": 44350, "original_path": "d:\\a01\\_work\\5\\s\\src\\vctools\\crt\\vcstartup\\build\\md\\msvcrt_kernel32\\predefined C++ types (compiler internal)", "line": 201, "module": null, "source_string_id": 44345}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] $_s__RTTIBaseClassArray$_extraBytes_16; TPI 0x110b
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_s__RTTIBaseClassArray$_extraBytes_16", "UniqueName": ".?AU$_s__RTTIBaseClassArray$_extraBytes_16@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [4366]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] $_s__RTTIBaseClassArray$_extraBytes_16; TPI 0x110e
// 原始定义文件映射: PDB_UDT_source_line
// size = 20; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 1, "Options": ["None", "HasUniqueName"], "FieldList": 4365, "Name": "$_s__RTTIBaseClassArray$_extraBytes_16", "UniqueName": ".?AU$_s__RTTIBaseClassArray$_extraBytes_16@@", "DerivationList": 0, "VTableShape": 0, "Size": 20}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4365, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4364, "FieldOffset": 0, "Name": "arrayOfBaseClassDescriptors"}}, "type_references": [{"index": 4364, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 4347, "IndexType": 35, "Size": 16, "Name": ""}}, "nested_types": {"ElementType": {"index": 4347, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4332, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4332, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 4331, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 4331, "kind": "LF_STRUCTURE", "spelling": "_s__RTTIBaseClassDescriptor", "status": "PDB_record_name"}}}}}}}]}
// [PDB UDT 源码行] {"ipi_index": 44351, "original_path": "d:\\a01\\_work\\5\\s\\src\\vctools\\crt\\vcstartup\\build\\md\\msvcrt_kernel32\\predefined C++ types (compiler internal)", "line": 197, "module": null, "source_string_id": 44345}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] $_TypeDescriptor$_extraBytes_24; TPI 0x1110
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_TypeDescriptor$_extraBytes_24", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_24@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [4371]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] $_TypeDescriptor$_extraBytes_24; TPI 0x1113
// 原始定义文件映射: PDB_UDT_source_line
// size = 40; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "HasUniqueName"], "FieldList": 4370, "Name": "$_TypeDescriptor$_extraBytes_24", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_24@@", "DerivationList": 0, "VTableShape": 0, "Size": 40}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4370, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4353, "FieldOffset": 0, "Name": "pVFTable"}}, "type_references": [{"index": 4353, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4352, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4352, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 3, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 3, "spelling": "Void", "status": "CodeView_simple_type; not Rust source spelling"}}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4370, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "spare"}}, "type_references": [{"index": 1539, "spelling": "Void / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4370, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4369, "FieldOffset": 16, "Name": "name"}}, "type_references": [{"index": 4369, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 112, "IndexType": 35, "Size": 24, "Name": ""}}, "nested_types": {"ElementType": {"index": 112, "spelling": "NarrowCharacter", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [PDB UDT 源码行] {"ipi_index": 44354, "original_path": "d:\\a01\\_work\\5\\s\\src\\vctools\\crt\\vcstartup\\build\\md\\msvcrt_kernel32\\predefined C++ types (compiler internal)", "line": 134, "module": null, "source_string_id": 44345}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] $_s__RTTIBaseClassArray$_extraBytes_8; TPI 0x1114
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_s__RTTIBaseClassArray$_extraBytes_8", "UniqueName": ".?AU$_s__RTTIBaseClassArray$_extraBytes_8@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [4375]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] $_s__RTTIBaseClassArray$_extraBytes_8; TPI 0x1117
// 原始定义文件映射: PDB_UDT_source_line
// size = 12; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 1, "Options": ["None", "HasUniqueName"], "FieldList": 4374, "Name": "$_s__RTTIBaseClassArray$_extraBytes_8", "UniqueName": ".?AU$_s__RTTIBaseClassArray$_extraBytes_8@@", "DerivationList": 0, "VTableShape": 0, "Size": 12}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4374, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4373, "FieldOffset": 0, "Name": "arrayOfBaseClassDescriptors"}}, "type_references": [{"index": 4373, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 4347, "IndexType": 35, "Size": 8, "Name": ""}}, "nested_types": {"ElementType": {"index": 4347, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4332, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4332, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 4331, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 4331, "kind": "LF_STRUCTURE", "spelling": "_s__RTTIBaseClassDescriptor", "status": "PDB_record_name"}}}}}}}]}
// [PDB UDT 源码行] {"ipi_index": 44355, "original_path": "d:\\a01\\_work\\5\\s\\src\\vctools\\crt\\vcstartup\\build\\md\\msvcrt_kernel32\\predefined C++ types (compiler internal)", "line": 197, "module": null, "source_string_id": 44345}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] _s__RTTIBaseClassArray; TPI 0x111a
// 原始定义文件映射: PDB_UDT_source_line
// size = 4; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 1, "Options": ["None", "HasUniqueName"], "FieldList": 4377, "Name": "_s__RTTIBaseClassArray", "UniqueName": ".?AU_s__RTTIBaseClassArray@@", "DerivationList": 0, "VTableShape": 0, "Size": 4}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4377, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4376, "FieldOffset": 0, "Name": "arrayOfBaseClassDescriptors"}}, "type_references": [{"index": 4376, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 4347, "IndexType": 35, "Size": 0, "Name": ""}}, "nested_types": {"ElementType": {"index": 4347, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4332, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4332, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 4331, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 4331, "kind": "LF_STRUCTURE", "spelling": "_s__RTTIBaseClassDescriptor", "status": "PDB_record_name"}}}}}}}]}
// [PDB UDT 源码行] {"ipi_index": 44357, "original_path": "d:\\a01\\_work\\5\\s\\src\\vctools\\crt\\vcstartup\\build\\md\\msvcrt_kernel32\\predefined C++ types (compiler internal)", "line": 197, "module": null, "source_string_id": 44345}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] $_TypeDescriptor$_extraBytes_21; TPI 0x111b
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_TypeDescriptor$_extraBytes_21", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_21@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [4382]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] $_TypeDescriptor$_extraBytes_21; TPI 0x111e
// 原始定义文件映射: PDB_UDT_source_line
// size = 37; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "HasUniqueName"], "FieldList": 4381, "Name": "$_TypeDescriptor$_extraBytes_21", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_21@@", "DerivationList": 0, "VTableShape": 0, "Size": 37}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4381, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4353, "FieldOffset": 0, "Name": "pVFTable"}}, "type_references": [{"index": 4353, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4352, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4352, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 3, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 3, "spelling": "Void", "status": "CodeView_simple_type; not Rust source spelling"}}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4381, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "spare"}}, "type_references": [{"index": 1539, "spelling": "Void / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4381, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4380, "FieldOffset": 16, "Name": "name"}}, "type_references": [{"index": 4380, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 112, "IndexType": 35, "Size": 21, "Name": ""}}, "nested_types": {"ElementType": {"index": 112, "spelling": "NarrowCharacter", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [PDB UDT 源码行] {"ipi_index": 44358, "original_path": "d:\\a01\\_work\\5\\s\\src\\vctools\\crt\\vcstartup\\build\\md\\msvcrt_kernel32\\predefined C++ types (compiler internal)", "line": 134, "module": null, "source_string_id": 44345}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] _PMD; TPI 0x1120
// 原始定义文件映射: PDB_UDT_source_line
// size = 12; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "HasUniqueName"], "FieldList": 4383, "Name": "_PMD", "UniqueName": ".?AU_PMD@@", "DerivationList": 0, "VTableShape": 0, "Size": 12}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4383, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 116, "FieldOffset": 0, "Name": "mdisp"}}, "type_references": [{"index": 116, "spelling": "Int32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4383, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 116, "FieldOffset": 4, "Name": "pdisp"}}, "type_references": [{"index": 116, "spelling": "Int32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4383, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 116, "FieldOffset": 8, "Name": "vdisp"}}, "type_references": [{"index": 116, "spelling": "Int32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 44360, "original_path": "d:\\a01\\_work\\5\\s\\src\\vctools\\crt\\vcstartup\\build\\md\\msvcrt_kernel32\\predefined C++ types (compiler internal)", "line": 123, "module": null, "source_string_id": 44345}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] $_TypeDescriptor$_extraBytes_19; TPI 0x1121
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_TypeDescriptor$_extraBytes_19", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_19@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [4388]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] $_TypeDescriptor$_extraBytes_19; TPI 0x1124
// 原始定义文件映射: PDB_UDT_source_line
// size = 35; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "HasUniqueName"], "FieldList": 4387, "Name": "$_TypeDescriptor$_extraBytes_19", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_19@@", "DerivationList": 0, "VTableShape": 0, "Size": 35}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4387, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4353, "FieldOffset": 0, "Name": "pVFTable"}}, "type_references": [{"index": 4353, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4352, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4352, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 3, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 3, "spelling": "Void", "status": "CodeView_simple_type; not Rust source spelling"}}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4387, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "spare"}}, "type_references": [{"index": 1539, "spelling": "Void / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4387, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4386, "FieldOffset": 16, "Name": "name"}}, "type_references": [{"index": 4386, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 112, "IndexType": 35, "Size": 19, "Name": ""}}, "nested_types": {"ElementType": {"index": 112, "spelling": "NarrowCharacter", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [PDB UDT 源码行] {"ipi_index": 44361, "original_path": "d:\\a01\\_work\\5\\s\\src\\vctools\\crt\\vcstartup\\build\\md\\msvcrt_kernel32\\predefined C++ types (compiler internal)", "line": 134, "module": null, "source_string_id": 44345}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] $_TypeDescriptor$_extraBytes_28; TPI 0x1125
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_TypeDescriptor$_extraBytes_28", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_28@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [4392]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] $_TypeDescriptor$_extraBytes_28; TPI 0x1128
// 原始定义文件映射: PDB_UDT_source_line
// size = 44; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "HasUniqueName"], "FieldList": 4391, "Name": "$_TypeDescriptor$_extraBytes_28", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_28@@", "DerivationList": 0, "VTableShape": 0, "Size": 44}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4391, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4353, "FieldOffset": 0, "Name": "pVFTable"}}, "type_references": [{"index": 4353, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4352, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4352, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 3, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 3, "spelling": "Void", "status": "CodeView_simple_type; not Rust source spelling"}}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4391, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "spare"}}, "type_references": [{"index": 1539, "spelling": "Void / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4391, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4390, "FieldOffset": 16, "Name": "name"}}, "type_references": [{"index": 4390, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 112, "IndexType": 35, "Size": 28, "Name": ""}}, "nested_types": {"ElementType": {"index": 112, "spelling": "NarrowCharacter", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [PDB UDT 源码行] {"ipi_index": 44372, "original_path": "d:\\a01\\_work\\5\\s\\src\\vctools\\crt\\vcstartup\\build\\md\\msvcrt_kernel32\\predefined C++ types (compiler internal)", "line": 134, "module": null, "source_string_id": 44345}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] _TypeDescriptor; TPI 0x112b
// 原始定义文件映射: PDB_UDT_source_line
// size = 16; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "HasUniqueName"], "FieldList": 4394, "Name": "_TypeDescriptor", "UniqueName": ".?AU_TypeDescriptor@@", "DerivationList": 0, "VTableShape": 0, "Size": 16}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4394, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4353, "FieldOffset": 0, "Name": "pVFTable"}}, "type_references": [{"index": 4353, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4352, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4352, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 3, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 3, "spelling": "Void", "status": "CodeView_simple_type; not Rust source spelling"}}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4394, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "spare"}}, "type_references": [{"index": 1539, "spelling": "Void / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4394, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4393, "FieldOffset": 16, "Name": "name"}}, "type_references": [{"index": 4393, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 112, "IndexType": 35, "Size": 0, "Name": ""}}, "nested_types": {"ElementType": {"index": 112, "spelling": "NarrowCharacter", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [PDB UDT 源码行] {"ipi_index": 44373, "original_path": "d:\\a01\\_work\\5\\s\\src\\vctools\\crt\\vcstartup\\build\\md\\msvcrt_kernel32\\predefined C++ types (compiler internal)", "line": 134, "module": null, "source_string_id": 44345}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] $_TypeDescriptor$_extraBytes_16; TPI 0x112c
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_TypeDescriptor$_extraBytes_16", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_16@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [4399]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] $_TypeDescriptor$_extraBytes_16; TPI 0x112f
// 原始定义文件映射: PDB_UDT_source_line
// size = 32; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "HasUniqueName"], "FieldList": 4398, "Name": "$_TypeDescriptor$_extraBytes_16", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_16@@", "DerivationList": 0, "VTableShape": 0, "Size": 32}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4398, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4353, "FieldOffset": 0, "Name": "pVFTable"}}, "type_references": [{"index": 4353, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4352, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4352, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 3, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 3, "spelling": "Void", "status": "CodeView_simple_type; not Rust source spelling"}}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4398, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "spare"}}, "type_references": [{"index": 1539, "spelling": "Void / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4398, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4397, "FieldOffset": 16, "Name": "name"}}, "type_references": [{"index": 4397, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 112, "IndexType": 35, "Size": 16, "Name": ""}}, "nested_types": {"ElementType": {"index": 112, "spelling": "NarrowCharacter", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [PDB UDT 源码行] {"ipi_index": 44374, "original_path": "d:\\a01\\_work\\5\\s\\src\\vctools\\crt\\vcstartup\\build\\md\\msvcrt_kernel32\\predefined C++ types (compiler internal)", "line": 134, "module": null, "source_string_id": 44345}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] $_TypeDescriptor$_extraBytes_31; TPI 0x1130
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_TypeDescriptor$_extraBytes_31", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_31@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [4403]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] $_TypeDescriptor$_extraBytes_31; TPI 0x1133
// 原始定义文件映射: PDB_UDT_source_line
// size = 47; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "HasUniqueName"], "FieldList": 4402, "Name": "$_TypeDescriptor$_extraBytes_31", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_31@@", "DerivationList": 0, "VTableShape": 0, "Size": 47}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4402, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4353, "FieldOffset": 0, "Name": "pVFTable"}}, "type_references": [{"index": 4353, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4352, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4352, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 3, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 3, "spelling": "Void", "status": "CodeView_simple_type; not Rust source spelling"}}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4402, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "spare"}}, "type_references": [{"index": 1539, "spelling": "Void / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4402, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4401, "FieldOffset": 16, "Name": "name"}}, "type_references": [{"index": 4401, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 112, "IndexType": 35, "Size": 31, "Name": ""}}, "nested_types": {"ElementType": {"index": 112, "spelling": "NarrowCharacter", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [PDB UDT 源码行] {"ipi_index": 44375, "original_path": "d:\\a01\\_work\\5\\s\\src\\vctools\\crt\\vcstartup\\build\\md\\msvcrt_kernel32\\predefined C++ types (compiler internal)", "line": 134, "module": null, "source_string_id": 44345}
// === 完整 TPI 结构关系引用记录；原文不转换成猜测的 Rust ABI ===
// [TPI 0x10eb] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "_s__RTTIBaseClassDescriptor", "UniqueName": ".?AU_s__RTTIBaseClassDescriptor@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x10ec] {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 4331, "Modifiers": ["None", "Const"]}}
// [TPI 0x10ed] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "_TypeDescriptor", "UniqueName": ".?AU_TypeDescriptor@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x10ee] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4333, "Attrs": 65548}}
// [TPI 0x10ef] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "_PMD", "UniqueName": ".?AU_PMD@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x10f0] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "_s__RTTIClassHierarchyDescriptor", "UniqueName": ".?AU_s__RTTIClassHierarchyDescriptor@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x10f1] {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 4336, "Modifiers": ["None", "Const"]}}
// [TPI 0x10f2] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4337, "Attrs": 65548}}
// [TPI 0x10f3] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4334, "FieldOffset": 0, "Name": "pTypeDescriptor"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 8, "Name": "numContainedBases"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4335, "FieldOffset": 12, "Name": "where"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 24, "Name": "attributes"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4338, "FieldOffset": 28, "Name": "pClassDescriptor"}}]}
// [TPI 0x10f4] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 5, "Options": ["None", "HasUniqueName"], "FieldList": 4339, "Name": "_s__RTTIBaseClassDescriptor", "UniqueName": ".?AU_s__RTTIBaseClassDescriptor@@", "DerivationList": 0, "VTableShape": 0, "Size": 36}}
// [TPI 0x10f5] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "_s__RTTICompleteObjectLocator2", "UniqueName": ".?AU_s__RTTICompleteObjectLocator2@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x10f6] {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 4341, "Modifiers": ["None", "Const"]}}
// [TPI 0x10f7] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4342, "Attrs": 65548}}
// [TPI 0x10f8] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 0, "Name": "signature"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 4, "Name": "offset"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 8, "Name": "cdOffset"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4334, "FieldOffset": 12, "Name": "pTypeDescriptor"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4338, "FieldOffset": 20, "Name": "pClassDescriptor"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4343, "FieldOffset": 28, "Name": "pSelf"}}]}
// [TPI 0x10f9] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 6, "Options": ["None", "HasUniqueName"], "FieldList": 4344, "Name": "_s__RTTICompleteObjectLocator2", "UniqueName": ".?AU_s__RTTICompleteObjectLocator2@@", "DerivationList": 0, "VTableShape": 0, "Size": 36}}
// [TPI 0x10fa] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_s__RTTIBaseClassArray$_extraBytes_24", "UniqueName": ".?AU$_s__RTTIBaseClassArray$_extraBytes_24@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x10fb] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4332, "Attrs": 65548}}
// [TPI 0x10fc] {"Kind": "LF_ARRAY", "Array": {"ElementType": 4347, "IndexType": 35, "Size": 24, "Name": ""}}
// [TPI 0x10fd] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4348, "FieldOffset": 0, "Name": "arrayOfBaseClassDescriptors"}}]}
// [TPI 0x10fe] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 1, "Options": ["None", "HasUniqueName"], "FieldList": 4349, "Name": "$_s__RTTIBaseClassArray$_extraBytes_24", "UniqueName": ".?AU$_s__RTTIBaseClassArray$_extraBytes_24@@", "DerivationList": 0, "VTableShape": 0, "Size": 28}}
// [TPI 0x10ff] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_TypeDescriptor$_extraBytes_20", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_20@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1100] {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 3, "Modifiers": ["None", "Const"]}}
// [TPI 0x1101] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4352, "Attrs": 65548}}
// [TPI 0x1102] {"Kind": "LF_ARRAY", "Array": {"ElementType": 112, "IndexType": 35, "Size": 20, "Name": ""}}
// [TPI 0x1103] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4353, "FieldOffset": 0, "Name": "pVFTable"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "spare"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4354, "FieldOffset": 16, "Name": "name"}}]}
// [TPI 0x1104] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "HasUniqueName"], "FieldList": 4355, "Name": "$_TypeDescriptor$_extraBytes_20", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_20@@", "DerivationList": 0, "VTableShape": 0, "Size": 36}}
// [TPI 0x1105] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "_s__RTTIBaseClassArray", "UniqueName": ".?AU_s__RTTIBaseClassArray@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1106] {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 4357, "Modifiers": ["None", "Const"]}}
// [TPI 0x1107] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4358, "Attrs": 65548}}
// [TPI 0x1108] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 0, "Name": "signature"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 4, "Name": "attributes"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 8, "Name": "numBaseClasses"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4359, "FieldOffset": 12, "Name": "pBaseClassArray"}}]}
// [TPI 0x1109] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None", "HasUniqueName"], "FieldList": 4360, "Name": "_s__RTTIClassHierarchyDescriptor", "UniqueName": ".?AU_s__RTTIClassHierarchyDescriptor@@", "DerivationList": 0, "VTableShape": 0, "Size": 20}}
// [TPI 0x110b] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_s__RTTIBaseClassArray$_extraBytes_16", "UniqueName": ".?AU$_s__RTTIBaseClassArray$_extraBytes_16@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x110c] {"Kind": "LF_ARRAY", "Array": {"ElementType": 4347, "IndexType": 35, "Size": 16, "Name": ""}}
// [TPI 0x110d] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4364, "FieldOffset": 0, "Name": "arrayOfBaseClassDescriptors"}}]}
// [TPI 0x110e] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 1, "Options": ["None", "HasUniqueName"], "FieldList": 4365, "Name": "$_s__RTTIBaseClassArray$_extraBytes_16", "UniqueName": ".?AU$_s__RTTIBaseClassArray$_extraBytes_16@@", "DerivationList": 0, "VTableShape": 0, "Size": 20}}
// [TPI 0x1110] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_TypeDescriptor$_extraBytes_24", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_24@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1111] {"Kind": "LF_ARRAY", "Array": {"ElementType": 112, "IndexType": 35, "Size": 24, "Name": ""}}
// [TPI 0x1112] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4353, "FieldOffset": 0, "Name": "pVFTable"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "spare"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4369, "FieldOffset": 16, "Name": "name"}}]}
// [TPI 0x1113] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "HasUniqueName"], "FieldList": 4370, "Name": "$_TypeDescriptor$_extraBytes_24", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_24@@", "DerivationList": 0, "VTableShape": 0, "Size": 40}}
// [TPI 0x1114] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_s__RTTIBaseClassArray$_extraBytes_8", "UniqueName": ".?AU$_s__RTTIBaseClassArray$_extraBytes_8@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1115] {"Kind": "LF_ARRAY", "Array": {"ElementType": 4347, "IndexType": 35, "Size": 8, "Name": ""}}
// [TPI 0x1116] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4373, "FieldOffset": 0, "Name": "arrayOfBaseClassDescriptors"}}]}
// [TPI 0x1117] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 1, "Options": ["None", "HasUniqueName"], "FieldList": 4374, "Name": "$_s__RTTIBaseClassArray$_extraBytes_8", "UniqueName": ".?AU$_s__RTTIBaseClassArray$_extraBytes_8@@", "DerivationList": 0, "VTableShape": 0, "Size": 12}}
// [TPI 0x1118] {"Kind": "LF_ARRAY", "Array": {"ElementType": 4347, "IndexType": 35, "Size": 0, "Name": ""}}
// [TPI 0x1119] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4376, "FieldOffset": 0, "Name": "arrayOfBaseClassDescriptors"}}]}
// [TPI 0x111a] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 1, "Options": ["None", "HasUniqueName"], "FieldList": 4377, "Name": "_s__RTTIBaseClassArray", "UniqueName": ".?AU_s__RTTIBaseClassArray@@", "DerivationList": 0, "VTableShape": 0, "Size": 4}}
// [TPI 0x111b] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_TypeDescriptor$_extraBytes_21", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_21@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x111c] {"Kind": "LF_ARRAY", "Array": {"ElementType": 112, "IndexType": 35, "Size": 21, "Name": ""}}
// [TPI 0x111d] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4353, "FieldOffset": 0, "Name": "pVFTable"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "spare"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4380, "FieldOffset": 16, "Name": "name"}}]}
// [TPI 0x111e] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "HasUniqueName"], "FieldList": 4381, "Name": "$_TypeDescriptor$_extraBytes_21", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_21@@", "DerivationList": 0, "VTableShape": 0, "Size": 37}}
// [TPI 0x111f] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 116, "FieldOffset": 0, "Name": "mdisp"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 116, "FieldOffset": 4, "Name": "pdisp"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 116, "FieldOffset": 8, "Name": "vdisp"}}]}
// [TPI 0x1120] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "HasUniqueName"], "FieldList": 4383, "Name": "_PMD", "UniqueName": ".?AU_PMD@@", "DerivationList": 0, "VTableShape": 0, "Size": 12}}
// [TPI 0x1121] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_TypeDescriptor$_extraBytes_19", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_19@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1122] {"Kind": "LF_ARRAY", "Array": {"ElementType": 112, "IndexType": 35, "Size": 19, "Name": ""}}
// [TPI 0x1123] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4353, "FieldOffset": 0, "Name": "pVFTable"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "spare"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4386, "FieldOffset": 16, "Name": "name"}}]}
// [TPI 0x1124] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "HasUniqueName"], "FieldList": 4387, "Name": "$_TypeDescriptor$_extraBytes_19", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_19@@", "DerivationList": 0, "VTableShape": 0, "Size": 35}}
// [TPI 0x1125] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_TypeDescriptor$_extraBytes_28", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_28@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1126] {"Kind": "LF_ARRAY", "Array": {"ElementType": 112, "IndexType": 35, "Size": 28, "Name": ""}}
// [TPI 0x1127] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4353, "FieldOffset": 0, "Name": "pVFTable"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "spare"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4390, "FieldOffset": 16, "Name": "name"}}]}
// [TPI 0x1128] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "HasUniqueName"], "FieldList": 4391, "Name": "$_TypeDescriptor$_extraBytes_28", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_28@@", "DerivationList": 0, "VTableShape": 0, "Size": 44}}
// [TPI 0x1129] {"Kind": "LF_ARRAY", "Array": {"ElementType": 112, "IndexType": 35, "Size": 0, "Name": ""}}
// [TPI 0x112a] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4353, "FieldOffset": 0, "Name": "pVFTable"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "spare"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4393, "FieldOffset": 16, "Name": "name"}}]}
// [TPI 0x112b] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "HasUniqueName"], "FieldList": 4394, "Name": "_TypeDescriptor", "UniqueName": ".?AU_TypeDescriptor@@", "DerivationList": 0, "VTableShape": 0, "Size": 16}}
// [TPI 0x112c] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_TypeDescriptor$_extraBytes_16", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_16@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x112d] {"Kind": "LF_ARRAY", "Array": {"ElementType": 112, "IndexType": 35, "Size": 16, "Name": ""}}
// [TPI 0x112e] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4353, "FieldOffset": 0, "Name": "pVFTable"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "spare"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4397, "FieldOffset": 16, "Name": "name"}}]}
// [TPI 0x112f] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "HasUniqueName"], "FieldList": 4398, "Name": "$_TypeDescriptor$_extraBytes_16", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_16@@", "DerivationList": 0, "VTableShape": 0, "Size": 32}}
// [TPI 0x1130] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_TypeDescriptor$_extraBytes_31", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_31@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1131] {"Kind": "LF_ARRAY", "Array": {"ElementType": 112, "IndexType": 35, "Size": 31, "Name": ""}}
// [TPI 0x1132] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4353, "FieldOffset": 0, "Name": "pVFTable"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "spare"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4401, "FieldOffset": 16, "Name": "name"}}]}
// [TPI 0x1133] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "HasUniqueName"], "FieldList": 4402, "Name": "$_TypeDescriptor$_extraBytes_31", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_31@@", "DerivationList": 0, "VTableShape": 0, "Size": 47}}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: predefined C++ types (compiler internal).rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===
