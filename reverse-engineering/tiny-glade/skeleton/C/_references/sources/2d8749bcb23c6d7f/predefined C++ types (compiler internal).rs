// Generated from PDB metadata by skeleton/_tools/fill_type_structures.py.
// Version: C
// 类型元数据；原作者源码文本/定义文件见逐项映射；不生成依赖实现。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: predefined C++ types (compiler internal).rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB forward_declaration_only] _s__RTTIBaseClassDescriptor; TPI 0x20b9
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "_s__RTTIBaseClassDescriptor", "UniqueName": ".?AU_s__RTTIBaseClassDescriptor@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [8386]
// ------------------------------------------------------------------
// [PDB forward_declaration_only] _TypeDescriptor; TPI 0x20bb
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "_TypeDescriptor", "UniqueName": ".?AU_TypeDescriptor@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [8439]
// ------------------------------------------------------------------
// [PDB forward_declaration_only] _PMD; TPI 0x20bd
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "_PMD", "UniqueName": ".?AU_PMD@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [8428]
// ------------------------------------------------------------------
// [PDB forward_declaration_only] _s__RTTIClassHierarchyDescriptor; TPI 0x20be
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "_s__RTTIClassHierarchyDescriptor", "UniqueName": ".?AU_s__RTTIClassHierarchyDescriptor@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [8405]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] _s__RTTIBaseClassDescriptor; TPI 0x20c2
// 原始定义文件映射: PDB_UDT_source_line
// size = 36; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 5, "Options": ["None", "HasUniqueName"], "FieldList": 8385, "Name": "_s__RTTIBaseClassDescriptor", "UniqueName": ".?AU_s__RTTIBaseClassDescriptor@@", "DerivationList": 0, "VTableShape": 0, "Size": 36}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8385, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8380, "FieldOffset": 0, "Name": "pTypeDescriptor"}}, "type_references": [{"index": 8380, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 8379, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 8379, "kind": "LF_STRUCTURE", "spelling": "_TypeDescriptor", "status": "PDB_record_name"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8385, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 8, "Name": "numContainedBases"}}, "type_references": [{"index": 34, "spelling": "UInt32Long", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8385, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8381, "FieldOffset": 12, "Name": "where"}}, "type_references": [{"index": 8381, "kind": "LF_STRUCTURE", "spelling": "_PMD", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8385, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 24, "Name": "attributes"}}, "type_references": [{"index": 34, "spelling": "UInt32Long", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8385, "order": 4, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8384, "FieldOffset": 28, "Name": "pClassDescriptor"}}, "type_references": [{"index": 8384, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 8383, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 8383, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 8382, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 8382, "kind": "LF_STRUCTURE", "spelling": "_s__RTTIClassHierarchyDescriptor", "status": "PDB_record_name"}}}}}]}
// [PDB UDT 源码行] {"ipi_index": 139693, "original_path": "D:\\a\\_work\\1\\s\\src\\vctools\\crt\\vcstartup\\build\\md\\predefined C++ types (compiler internal)", "line": 199, "module": null, "source_string_id": 139692}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] _s__RTTICompleteObjectLocator2; TPI 0x20c3
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "_s__RTTICompleteObjectLocator2", "UniqueName": ".?AU_s__RTTICompleteObjectLocator2@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [8391]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] _s__RTTICompleteObjectLocator2; TPI 0x20c7
// 原始定义文件映射: PDB_UDT_source_line
// size = 36; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 6, "Options": ["None", "HasUniqueName"], "FieldList": 8390, "Name": "_s__RTTICompleteObjectLocator2", "UniqueName": ".?AU_s__RTTICompleteObjectLocator2@@", "DerivationList": 0, "VTableShape": 0, "Size": 36}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8390, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 0, "Name": "signature"}}, "type_references": [{"index": 34, "spelling": "UInt32Long", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8390, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 4, "Name": "offset"}}, "type_references": [{"index": 34, "spelling": "UInt32Long", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8390, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 8, "Name": "cdOffset"}}, "type_references": [{"index": 34, "spelling": "UInt32Long", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8390, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8380, "FieldOffset": 12, "Name": "pTypeDescriptor"}}, "type_references": [{"index": 8380, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 8379, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 8379, "kind": "LF_STRUCTURE", "spelling": "_TypeDescriptor", "status": "PDB_record_name"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8390, "order": 4, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8384, "FieldOffset": 20, "Name": "pClassDescriptor"}}, "type_references": [{"index": 8384, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 8383, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 8383, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 8382, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 8382, "kind": "LF_STRUCTURE", "spelling": "_s__RTTIClassHierarchyDescriptor", "status": "PDB_record_name"}}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8390, "order": 5, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8389, "FieldOffset": 28, "Name": "pSelf"}}, "type_references": [{"index": 8389, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 8388, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 8388, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 8387, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 8387, "kind": "LF_STRUCTURE", "spelling": "_s__RTTICompleteObjectLocator2", "status": "PDB_record_name"}}}}}]}
// [PDB UDT 源码行] {"ipi_index": 139694, "original_path": "D:\\a\\_work\\1\\s\\src\\vctools\\crt\\vcstartup\\build\\md\\predefined C++ types (compiler internal)", "line": 218, "module": null, "source_string_id": 139692}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] $_s__RTTIBaseClassArray$_extraBytes_24; TPI 0x20c8
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_s__RTTIBaseClassArray$_extraBytes_24", "UniqueName": ".?AU$_s__RTTIBaseClassArray$_extraBytes_24@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [8396]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] $_s__RTTIBaseClassArray$_extraBytes_24; TPI 0x20cc
// 原始定义文件映射: PDB_UDT_source_line
// size = 28; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 1, "Options": ["None", "HasUniqueName"], "FieldList": 8395, "Name": "$_s__RTTIBaseClassArray$_extraBytes_24", "UniqueName": ".?AU$_s__RTTIBaseClassArray$_extraBytes_24@@", "DerivationList": 0, "VTableShape": 0, "Size": 28}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8395, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8394, "FieldOffset": 0, "Name": "arrayOfBaseClassDescriptors"}}, "type_references": [{"index": 8394, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 8393, "IndexType": 35, "Size": 24, "Name": ""}}, "nested_types": {"ElementType": {"index": 8393, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 8378, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 8378, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 8377, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 8377, "kind": "LF_STRUCTURE", "spelling": "_s__RTTIBaseClassDescriptor", "status": "PDB_record_name"}}}}}}}]}
// [PDB UDT 源码行] {"ipi_index": 139695, "original_path": "D:\\a\\_work\\1\\s\\src\\vctools\\crt\\vcstartup\\build\\md\\predefined C++ types (compiler internal)", "line": 208, "module": null, "source_string_id": 139692}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] $_TypeDescriptor$_extraBytes_20; TPI 0x20cd
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_TypeDescriptor$_extraBytes_20", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_20@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [8400]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] $_TypeDescriptor$_extraBytes_20; TPI 0x20d0
// 原始定义文件映射: PDB_UDT_source_line
// size = 36; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "HasUniqueName"], "FieldList": 8399, "Name": "$_TypeDescriptor$_extraBytes_20", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_20@@", "DerivationList": 0, "VTableShape": 0, "Size": 36}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8399, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4125, "FieldOffset": 0, "Name": "pVFTable"}}, "type_references": [{"index": 4125, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4124, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4124, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 3, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 3, "spelling": "Void", "status": "CodeView_simple_type; not Rust source spelling"}}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8399, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "spare"}}, "type_references": [{"index": 1539, "spelling": "Void / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8399, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8398, "FieldOffset": 16, "Name": "name"}}, "type_references": [{"index": 8398, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 112, "IndexType": 35, "Size": 20, "Name": ""}}, "nested_types": {"ElementType": {"index": 112, "spelling": "NarrowCharacter", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [PDB UDT 源码行] {"ipi_index": 139696, "original_path": "D:\\a\\_work\\1\\s\\src\\vctools\\crt\\vcstartup\\build\\md\\predefined C++ types (compiler internal)", "line": 144, "module": null, "source_string_id": 139692}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] _s__RTTIBaseClassArray; TPI 0x20d1
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "_s__RTTIBaseClassArray", "UniqueName": ".?AU_s__RTTIBaseClassArray@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [8422]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] _s__RTTIClassHierarchyDescriptor; TPI 0x20d5
// 原始定义文件映射: PDB_UDT_source_line
// size = 20; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None", "HasUniqueName"], "FieldList": 8404, "Name": "_s__RTTIClassHierarchyDescriptor", "UniqueName": ".?AU_s__RTTIClassHierarchyDescriptor@@", "DerivationList": 0, "VTableShape": 0, "Size": 20}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8404, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 0, "Name": "signature"}}, "type_references": [{"index": 34, "spelling": "UInt32Long", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8404, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 4, "Name": "attributes"}}, "type_references": [{"index": 34, "spelling": "UInt32Long", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8404, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 8, "Name": "numBaseClasses"}}, "type_references": [{"index": 34, "spelling": "UInt32Long", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8404, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8403, "FieldOffset": 12, "Name": "pBaseClassArray"}}, "type_references": [{"index": 8403, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 8402, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 8402, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 8401, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 8401, "kind": "LF_STRUCTURE", "spelling": "_s__RTTIBaseClassArray", "status": "PDB_record_name"}}}}}]}
// [PDB UDT 源码行] {"ipi_index": 139697, "original_path": "D:\\a\\_work\\1\\s\\src\\vctools\\crt\\vcstartup\\build\\md\\predefined C++ types (compiler internal)", "line": 212, "module": null, "source_string_id": 139692}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] $_s__RTTIBaseClassArray$_extraBytes_16; TPI 0x20d7
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_s__RTTIBaseClassArray$_extraBytes_16", "UniqueName": ".?AU$_s__RTTIBaseClassArray$_extraBytes_16@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [8410]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] $_s__RTTIBaseClassArray$_extraBytes_16; TPI 0x20da
// 原始定义文件映射: PDB_UDT_source_line
// size = 20; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 1, "Options": ["None", "HasUniqueName"], "FieldList": 8409, "Name": "$_s__RTTIBaseClassArray$_extraBytes_16", "UniqueName": ".?AU$_s__RTTIBaseClassArray$_extraBytes_16@@", "DerivationList": 0, "VTableShape": 0, "Size": 20}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8409, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8408, "FieldOffset": 0, "Name": "arrayOfBaseClassDescriptors"}}, "type_references": [{"index": 8408, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 8393, "IndexType": 35, "Size": 16, "Name": ""}}, "nested_types": {"ElementType": {"index": 8393, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 8378, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 8378, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 8377, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 8377, "kind": "LF_STRUCTURE", "spelling": "_s__RTTIBaseClassDescriptor", "status": "PDB_record_name"}}}}}}}]}
// [PDB UDT 源码行] {"ipi_index": 139698, "original_path": "D:\\a\\_work\\1\\s\\src\\vctools\\crt\\vcstartup\\build\\md\\predefined C++ types (compiler internal)", "line": 208, "module": null, "source_string_id": 139692}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] $_TypeDescriptor$_extraBytes_24; TPI 0x20dc
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_TypeDescriptor$_extraBytes_24", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_24@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [8415]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] $_TypeDescriptor$_extraBytes_24; TPI 0x20df
// 原始定义文件映射: PDB_UDT_source_line
// size = 40; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "HasUniqueName"], "FieldList": 8414, "Name": "$_TypeDescriptor$_extraBytes_24", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_24@@", "DerivationList": 0, "VTableShape": 0, "Size": 40}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8414, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4125, "FieldOffset": 0, "Name": "pVFTable"}}, "type_references": [{"index": 4125, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4124, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4124, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 3, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 3, "spelling": "Void", "status": "CodeView_simple_type; not Rust source spelling"}}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8414, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "spare"}}, "type_references": [{"index": 1539, "spelling": "Void / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8414, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8413, "FieldOffset": 16, "Name": "name"}}, "type_references": [{"index": 8413, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 112, "IndexType": 35, "Size": 24, "Name": ""}}, "nested_types": {"ElementType": {"index": 112, "spelling": "NarrowCharacter", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [PDB UDT 源码行] {"ipi_index": 139701, "original_path": "D:\\a\\_work\\1\\s\\src\\vctools\\crt\\vcstartup\\build\\md\\predefined C++ types (compiler internal)", "line": 144, "module": null, "source_string_id": 139692}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] $_s__RTTIBaseClassArray$_extraBytes_8; TPI 0x20e0
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_s__RTTIBaseClassArray$_extraBytes_8", "UniqueName": ".?AU$_s__RTTIBaseClassArray$_extraBytes_8@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [8419]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] $_s__RTTIBaseClassArray$_extraBytes_8; TPI 0x20e3
// 原始定义文件映射: PDB_UDT_source_line
// size = 12; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 1, "Options": ["None", "HasUniqueName"], "FieldList": 8418, "Name": "$_s__RTTIBaseClassArray$_extraBytes_8", "UniqueName": ".?AU$_s__RTTIBaseClassArray$_extraBytes_8@@", "DerivationList": 0, "VTableShape": 0, "Size": 12}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8418, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8417, "FieldOffset": 0, "Name": "arrayOfBaseClassDescriptors"}}, "type_references": [{"index": 8417, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 8393, "IndexType": 35, "Size": 8, "Name": ""}}, "nested_types": {"ElementType": {"index": 8393, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 8378, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 8378, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 8377, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 8377, "kind": "LF_STRUCTURE", "spelling": "_s__RTTIBaseClassDescriptor", "status": "PDB_record_name"}}}}}}}]}
// [PDB UDT 源码行] {"ipi_index": 139702, "original_path": "D:\\a\\_work\\1\\s\\src\\vctools\\crt\\vcstartup\\build\\md\\predefined C++ types (compiler internal)", "line": 208, "module": null, "source_string_id": 139692}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] _s__RTTIBaseClassArray; TPI 0x20e6
// 原始定义文件映射: PDB_UDT_source_line
// size = 4; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 1, "Options": ["None", "HasUniqueName"], "FieldList": 8421, "Name": "_s__RTTIBaseClassArray", "UniqueName": ".?AU_s__RTTIBaseClassArray@@", "DerivationList": 0, "VTableShape": 0, "Size": 4}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8421, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8420, "FieldOffset": 0, "Name": "arrayOfBaseClassDescriptors"}}, "type_references": [{"index": 8420, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 8393, "IndexType": 35, "Size": 0, "Name": ""}}, "nested_types": {"ElementType": {"index": 8393, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 8378, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 8378, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 8377, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 8377, "kind": "LF_STRUCTURE", "spelling": "_s__RTTIBaseClassDescriptor", "status": "PDB_record_name"}}}}}}}]}
// [PDB UDT 源码行] {"ipi_index": 139704, "original_path": "D:\\a\\_work\\1\\s\\src\\vctools\\crt\\vcstartup\\build\\md\\predefined C++ types (compiler internal)", "line": 208, "module": null, "source_string_id": 139692}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] $_TypeDescriptor$_extraBytes_21; TPI 0x20e7
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_TypeDescriptor$_extraBytes_21", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_21@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [8426]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] $_TypeDescriptor$_extraBytes_21; TPI 0x20ea
// 原始定义文件映射: PDB_UDT_source_line
// size = 37; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "HasUniqueName"], "FieldList": 8425, "Name": "$_TypeDescriptor$_extraBytes_21", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_21@@", "DerivationList": 0, "VTableShape": 0, "Size": 37}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8425, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4125, "FieldOffset": 0, "Name": "pVFTable"}}, "type_references": [{"index": 4125, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4124, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4124, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 3, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 3, "spelling": "Void", "status": "CodeView_simple_type; not Rust source spelling"}}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8425, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "spare"}}, "type_references": [{"index": 1539, "spelling": "Void / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8425, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8424, "FieldOffset": 16, "Name": "name"}}, "type_references": [{"index": 8424, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 112, "IndexType": 35, "Size": 21, "Name": ""}}, "nested_types": {"ElementType": {"index": 112, "spelling": "NarrowCharacter", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [PDB UDT 源码行] {"ipi_index": 139705, "original_path": "D:\\a\\_work\\1\\s\\src\\vctools\\crt\\vcstartup\\build\\md\\predefined C++ types (compiler internal)", "line": 144, "module": null, "source_string_id": 139692}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] _PMD; TPI 0x20ec
// 原始定义文件映射: PDB_UDT_source_line
// size = 12; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "HasUniqueName"], "FieldList": 8427, "Name": "_PMD", "UniqueName": ".?AU_PMD@@", "DerivationList": 0, "VTableShape": 0, "Size": 12}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8427, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 116, "FieldOffset": 0, "Name": "mdisp"}}, "type_references": [{"index": 116, "spelling": "Int32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8427, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 116, "FieldOffset": 4, "Name": "pdisp"}}, "type_references": [{"index": 116, "spelling": "Int32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8427, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 116, "FieldOffset": 8, "Name": "vdisp"}}, "type_references": [{"index": 116, "spelling": "Int32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 139707, "original_path": "D:\\a\\_work\\1\\s\\src\\vctools\\crt\\vcstartup\\build\\md\\predefined C++ types (compiler internal)", "line": 133, "module": null, "source_string_id": 139692}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] $_TypeDescriptor$_extraBytes_19; TPI 0x20ed
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_TypeDescriptor$_extraBytes_19", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_19@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [8432]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] $_TypeDescriptor$_extraBytes_19; TPI 0x20f0
// 原始定义文件映射: PDB_UDT_source_line
// size = 35; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "HasUniqueName"], "FieldList": 8431, "Name": "$_TypeDescriptor$_extraBytes_19", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_19@@", "DerivationList": 0, "VTableShape": 0, "Size": 35}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8431, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4125, "FieldOffset": 0, "Name": "pVFTable"}}, "type_references": [{"index": 4125, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4124, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4124, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 3, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 3, "spelling": "Void", "status": "CodeView_simple_type; not Rust source spelling"}}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8431, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "spare"}}, "type_references": [{"index": 1539, "spelling": "Void / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8431, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8430, "FieldOffset": 16, "Name": "name"}}, "type_references": [{"index": 8430, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 112, "IndexType": 35, "Size": 19, "Name": ""}}, "nested_types": {"ElementType": {"index": 112, "spelling": "NarrowCharacter", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [PDB UDT 源码行] {"ipi_index": 139708, "original_path": "D:\\a\\_work\\1\\s\\src\\vctools\\crt\\vcstartup\\build\\md\\predefined C++ types (compiler internal)", "line": 144, "module": null, "source_string_id": 139692}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] $_TypeDescriptor$_extraBytes_28; TPI 0x20f1
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_TypeDescriptor$_extraBytes_28", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_28@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [8436]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] $_TypeDescriptor$_extraBytes_28; TPI 0x20f4
// 原始定义文件映射: PDB_UDT_source_line
// size = 44; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "HasUniqueName"], "FieldList": 8435, "Name": "$_TypeDescriptor$_extraBytes_28", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_28@@", "DerivationList": 0, "VTableShape": 0, "Size": 44}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8435, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4125, "FieldOffset": 0, "Name": "pVFTable"}}, "type_references": [{"index": 4125, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4124, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4124, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 3, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 3, "spelling": "Void", "status": "CodeView_simple_type; not Rust source spelling"}}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8435, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "spare"}}, "type_references": [{"index": 1539, "spelling": "Void / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8435, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8434, "FieldOffset": 16, "Name": "name"}}, "type_references": [{"index": 8434, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 112, "IndexType": 35, "Size": 28, "Name": ""}}, "nested_types": {"ElementType": {"index": 112, "spelling": "NarrowCharacter", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [PDB UDT 源码行] {"ipi_index": 139719, "original_path": "D:\\a\\_work\\1\\s\\src\\vctools\\crt\\vcstartup\\build\\md\\predefined C++ types (compiler internal)", "line": 144, "module": null, "source_string_id": 139692}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] _TypeDescriptor; TPI 0x20f7
// 原始定义文件映射: PDB_UDT_source_line
// size = 16; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "HasUniqueName"], "FieldList": 8438, "Name": "_TypeDescriptor", "UniqueName": ".?AU_TypeDescriptor@@", "DerivationList": 0, "VTableShape": 0, "Size": 16}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8438, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4125, "FieldOffset": 0, "Name": "pVFTable"}}, "type_references": [{"index": 4125, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4124, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4124, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 3, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 3, "spelling": "Void", "status": "CodeView_simple_type; not Rust source spelling"}}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8438, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "spare"}}, "type_references": [{"index": 1539, "spelling": "Void / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8438, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8437, "FieldOffset": 16, "Name": "name"}}, "type_references": [{"index": 8437, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 112, "IndexType": 35, "Size": 0, "Name": ""}}, "nested_types": {"ElementType": {"index": 112, "spelling": "NarrowCharacter", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [PDB UDT 源码行] {"ipi_index": 139720, "original_path": "D:\\a\\_work\\1\\s\\src\\vctools\\crt\\vcstartup\\build\\md\\predefined C++ types (compiler internal)", "line": 144, "module": null, "source_string_id": 139692}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] $_TypeDescriptor$_extraBytes_16; TPI 0x20f8
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_TypeDescriptor$_extraBytes_16", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_16@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [8442]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] $_TypeDescriptor$_extraBytes_16; TPI 0x20fa
// 原始定义文件映射: PDB_UDT_source_line
// size = 32; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "HasUniqueName"], "FieldList": 8441, "Name": "$_TypeDescriptor$_extraBytes_16", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_16@@", "DerivationList": 0, "VTableShape": 0, "Size": 32}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8441, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4125, "FieldOffset": 0, "Name": "pVFTable"}}, "type_references": [{"index": 4125, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4124, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4124, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 3, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 3, "spelling": "Void", "status": "CodeView_simple_type; not Rust source spelling"}}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8441, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "spare"}}, "type_references": [{"index": 1539, "spelling": "Void / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8441, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6930, "FieldOffset": 16, "Name": "name"}}, "type_references": [{"index": 6930, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 112, "IndexType": 35, "Size": 16, "Name": ""}}, "nested_types": {"ElementType": {"index": 112, "spelling": "NarrowCharacter", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [PDB UDT 源码行] {"ipi_index": 139721, "original_path": "D:\\a\\_work\\1\\s\\src\\vctools\\crt\\vcstartup\\build\\md\\predefined C++ types (compiler internal)", "line": 144, "module": null, "source_string_id": 139692}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] $_TypeDescriptor$_extraBytes_31; TPI 0x20fb
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_TypeDescriptor$_extraBytes_31", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_31@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [8446]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] $_TypeDescriptor$_extraBytes_31; TPI 0x20fe
// 原始定义文件映射: PDB_UDT_source_line
// size = 47; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "HasUniqueName"], "FieldList": 8445, "Name": "$_TypeDescriptor$_extraBytes_31", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_31@@", "DerivationList": 0, "VTableShape": 0, "Size": 47}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8445, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4125, "FieldOffset": 0, "Name": "pVFTable"}}, "type_references": [{"index": 4125, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4124, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4124, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 3, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 3, "spelling": "Void", "status": "CodeView_simple_type; not Rust source spelling"}}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8445, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "spare"}}, "type_references": [{"index": 1539, "spelling": "Void / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8445, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8444, "FieldOffset": 16, "Name": "name"}}, "type_references": [{"index": 8444, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 112, "IndexType": 35, "Size": 31, "Name": ""}}, "nested_types": {"ElementType": {"index": 112, "spelling": "NarrowCharacter", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [PDB UDT 源码行] {"ipi_index": 139722, "original_path": "D:\\a\\_work\\1\\s\\src\\vctools\\crt\\vcstartup\\build\\md\\predefined C++ types (compiler internal)", "line": 144, "module": null, "source_string_id": 139692}
// === 完整 TPI 结构关系引用记录；原文不转换成猜测的 Rust ABI ===
// [TPI 0x101c] {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 3, "Modifiers": ["None", "Const"]}}
// [TPI 0x101d] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4124, "Attrs": 65548}}
// [TPI 0x1b12] {"Kind": "LF_ARRAY", "Array": {"ElementType": 112, "IndexType": 35, "Size": 16, "Name": ""}}
// [TPI 0x20b9] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "_s__RTTIBaseClassDescriptor", "UniqueName": ".?AU_s__RTTIBaseClassDescriptor@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x20ba] {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 8377, "Modifiers": ["None", "Const"]}}
// [TPI 0x20bb] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "_TypeDescriptor", "UniqueName": ".?AU_TypeDescriptor@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x20bc] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 8379, "Attrs": 65548}}
// [TPI 0x20bd] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "_PMD", "UniqueName": ".?AU_PMD@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x20be] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "_s__RTTIClassHierarchyDescriptor", "UniqueName": ".?AU_s__RTTIClassHierarchyDescriptor@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x20bf] {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 8382, "Modifiers": ["None", "Const"]}}
// [TPI 0x20c0] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 8383, "Attrs": 65548}}
// [TPI 0x20c1] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8380, "FieldOffset": 0, "Name": "pTypeDescriptor"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 8, "Name": "numContainedBases"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8381, "FieldOffset": 12, "Name": "where"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 24, "Name": "attributes"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8384, "FieldOffset": 28, "Name": "pClassDescriptor"}}]}
// [TPI 0x20c2] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 5, "Options": ["None", "HasUniqueName"], "FieldList": 8385, "Name": "_s__RTTIBaseClassDescriptor", "UniqueName": ".?AU_s__RTTIBaseClassDescriptor@@", "DerivationList": 0, "VTableShape": 0, "Size": 36}}
// [TPI 0x20c3] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "_s__RTTICompleteObjectLocator2", "UniqueName": ".?AU_s__RTTICompleteObjectLocator2@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x20c4] {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 8387, "Modifiers": ["None", "Const"]}}
// [TPI 0x20c5] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 8388, "Attrs": 65548}}
// [TPI 0x20c6] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 0, "Name": "signature"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 4, "Name": "offset"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 8, "Name": "cdOffset"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8380, "FieldOffset": 12, "Name": "pTypeDescriptor"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8384, "FieldOffset": 20, "Name": "pClassDescriptor"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8389, "FieldOffset": 28, "Name": "pSelf"}}]}
// [TPI 0x20c7] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 6, "Options": ["None", "HasUniqueName"], "FieldList": 8390, "Name": "_s__RTTICompleteObjectLocator2", "UniqueName": ".?AU_s__RTTICompleteObjectLocator2@@", "DerivationList": 0, "VTableShape": 0, "Size": 36}}
// [TPI 0x20c8] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_s__RTTIBaseClassArray$_extraBytes_24", "UniqueName": ".?AU$_s__RTTIBaseClassArray$_extraBytes_24@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x20c9] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 8378, "Attrs": 65548}}
// [TPI 0x20ca] {"Kind": "LF_ARRAY", "Array": {"ElementType": 8393, "IndexType": 35, "Size": 24, "Name": ""}}
// [TPI 0x20cb] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8394, "FieldOffset": 0, "Name": "arrayOfBaseClassDescriptors"}}]}
// [TPI 0x20cc] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 1, "Options": ["None", "HasUniqueName"], "FieldList": 8395, "Name": "$_s__RTTIBaseClassArray$_extraBytes_24", "UniqueName": ".?AU$_s__RTTIBaseClassArray$_extraBytes_24@@", "DerivationList": 0, "VTableShape": 0, "Size": 28}}
// [TPI 0x20cd] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_TypeDescriptor$_extraBytes_20", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_20@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x20ce] {"Kind": "LF_ARRAY", "Array": {"ElementType": 112, "IndexType": 35, "Size": 20, "Name": ""}}
// [TPI 0x20cf] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4125, "FieldOffset": 0, "Name": "pVFTable"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "spare"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8398, "FieldOffset": 16, "Name": "name"}}]}
// [TPI 0x20d0] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "HasUniqueName"], "FieldList": 8399, "Name": "$_TypeDescriptor$_extraBytes_20", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_20@@", "DerivationList": 0, "VTableShape": 0, "Size": 36}}
// [TPI 0x20d1] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "_s__RTTIBaseClassArray", "UniqueName": ".?AU_s__RTTIBaseClassArray@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x20d2] {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 8401, "Modifiers": ["None", "Const"]}}
// [TPI 0x20d3] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 8402, "Attrs": 65548}}
// [TPI 0x20d4] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 0, "Name": "signature"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 4, "Name": "attributes"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 8, "Name": "numBaseClasses"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8403, "FieldOffset": 12, "Name": "pBaseClassArray"}}]}
// [TPI 0x20d5] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None", "HasUniqueName"], "FieldList": 8404, "Name": "_s__RTTIClassHierarchyDescriptor", "UniqueName": ".?AU_s__RTTIClassHierarchyDescriptor@@", "DerivationList": 0, "VTableShape": 0, "Size": 20}}
// [TPI 0x20d7] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_s__RTTIBaseClassArray$_extraBytes_16", "UniqueName": ".?AU$_s__RTTIBaseClassArray$_extraBytes_16@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x20d8] {"Kind": "LF_ARRAY", "Array": {"ElementType": 8393, "IndexType": 35, "Size": 16, "Name": ""}}
// [TPI 0x20d9] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8408, "FieldOffset": 0, "Name": "arrayOfBaseClassDescriptors"}}]}
// [TPI 0x20da] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 1, "Options": ["None", "HasUniqueName"], "FieldList": 8409, "Name": "$_s__RTTIBaseClassArray$_extraBytes_16", "UniqueName": ".?AU$_s__RTTIBaseClassArray$_extraBytes_16@@", "DerivationList": 0, "VTableShape": 0, "Size": 20}}
// [TPI 0x20dc] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_TypeDescriptor$_extraBytes_24", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_24@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x20dd] {"Kind": "LF_ARRAY", "Array": {"ElementType": 112, "IndexType": 35, "Size": 24, "Name": ""}}
// [TPI 0x20de] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4125, "FieldOffset": 0, "Name": "pVFTable"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "spare"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8413, "FieldOffset": 16, "Name": "name"}}]}
// [TPI 0x20df] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "HasUniqueName"], "FieldList": 8414, "Name": "$_TypeDescriptor$_extraBytes_24", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_24@@", "DerivationList": 0, "VTableShape": 0, "Size": 40}}
// [TPI 0x20e0] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_s__RTTIBaseClassArray$_extraBytes_8", "UniqueName": ".?AU$_s__RTTIBaseClassArray$_extraBytes_8@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x20e1] {"Kind": "LF_ARRAY", "Array": {"ElementType": 8393, "IndexType": 35, "Size": 8, "Name": ""}}
// [TPI 0x20e2] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8417, "FieldOffset": 0, "Name": "arrayOfBaseClassDescriptors"}}]}
// [TPI 0x20e3] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 1, "Options": ["None", "HasUniqueName"], "FieldList": 8418, "Name": "$_s__RTTIBaseClassArray$_extraBytes_8", "UniqueName": ".?AU$_s__RTTIBaseClassArray$_extraBytes_8@@", "DerivationList": 0, "VTableShape": 0, "Size": 12}}
// [TPI 0x20e4] {"Kind": "LF_ARRAY", "Array": {"ElementType": 8393, "IndexType": 35, "Size": 0, "Name": ""}}
// [TPI 0x20e5] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8420, "FieldOffset": 0, "Name": "arrayOfBaseClassDescriptors"}}]}
// [TPI 0x20e6] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 1, "Options": ["None", "HasUniqueName"], "FieldList": 8421, "Name": "_s__RTTIBaseClassArray", "UniqueName": ".?AU_s__RTTIBaseClassArray@@", "DerivationList": 0, "VTableShape": 0, "Size": 4}}
// [TPI 0x20e7] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_TypeDescriptor$_extraBytes_21", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_21@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x20e8] {"Kind": "LF_ARRAY", "Array": {"ElementType": 112, "IndexType": 35, "Size": 21, "Name": ""}}
// [TPI 0x20e9] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4125, "FieldOffset": 0, "Name": "pVFTable"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "spare"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8424, "FieldOffset": 16, "Name": "name"}}]}
// [TPI 0x20ea] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "HasUniqueName"], "FieldList": 8425, "Name": "$_TypeDescriptor$_extraBytes_21", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_21@@", "DerivationList": 0, "VTableShape": 0, "Size": 37}}
// [TPI 0x20eb] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 116, "FieldOffset": 0, "Name": "mdisp"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 116, "FieldOffset": 4, "Name": "pdisp"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 116, "FieldOffset": 8, "Name": "vdisp"}}]}
// [TPI 0x20ec] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "HasUniqueName"], "FieldList": 8427, "Name": "_PMD", "UniqueName": ".?AU_PMD@@", "DerivationList": 0, "VTableShape": 0, "Size": 12}}
// [TPI 0x20ed] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_TypeDescriptor$_extraBytes_19", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_19@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x20ee] {"Kind": "LF_ARRAY", "Array": {"ElementType": 112, "IndexType": 35, "Size": 19, "Name": ""}}
// [TPI 0x20ef] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4125, "FieldOffset": 0, "Name": "pVFTable"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "spare"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8430, "FieldOffset": 16, "Name": "name"}}]}
// [TPI 0x20f0] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "HasUniqueName"], "FieldList": 8431, "Name": "$_TypeDescriptor$_extraBytes_19", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_19@@", "DerivationList": 0, "VTableShape": 0, "Size": 35}}
// [TPI 0x20f1] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_TypeDescriptor$_extraBytes_28", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_28@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x20f2] {"Kind": "LF_ARRAY", "Array": {"ElementType": 112, "IndexType": 35, "Size": 28, "Name": ""}}
// [TPI 0x20f3] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4125, "FieldOffset": 0, "Name": "pVFTable"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "spare"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8434, "FieldOffset": 16, "Name": "name"}}]}
// [TPI 0x20f4] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "HasUniqueName"], "FieldList": 8435, "Name": "$_TypeDescriptor$_extraBytes_28", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_28@@", "DerivationList": 0, "VTableShape": 0, "Size": 44}}
// [TPI 0x20f5] {"Kind": "LF_ARRAY", "Array": {"ElementType": 112, "IndexType": 35, "Size": 0, "Name": ""}}
// [TPI 0x20f6] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4125, "FieldOffset": 0, "Name": "pVFTable"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "spare"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8437, "FieldOffset": 16, "Name": "name"}}]}
// [TPI 0x20f7] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "HasUniqueName"], "FieldList": 8438, "Name": "_TypeDescriptor", "UniqueName": ".?AU_TypeDescriptor@@", "DerivationList": 0, "VTableShape": 0, "Size": 16}}
// [TPI 0x20f8] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_TypeDescriptor$_extraBytes_16", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_16@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x20f9] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4125, "FieldOffset": 0, "Name": "pVFTable"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "spare"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6930, "FieldOffset": 16, "Name": "name"}}]}
// [TPI 0x20fa] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "HasUniqueName"], "FieldList": 8441, "Name": "$_TypeDescriptor$_extraBytes_16", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_16@@", "DerivationList": 0, "VTableShape": 0, "Size": 32}}
// [TPI 0x20fb] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_TypeDescriptor$_extraBytes_31", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_31@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x20fc] {"Kind": "LF_ARRAY", "Array": {"ElementType": 112, "IndexType": 35, "Size": 31, "Name": ""}}
// [TPI 0x20fd] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4125, "FieldOffset": 0, "Name": "pVFTable"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "spare"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8444, "FieldOffset": 16, "Name": "name"}}]}
// [TPI 0x20fe] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "HasUniqueName"], "FieldList": 8445, "Name": "$_TypeDescriptor$_extraBytes_31", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_31@@", "DerivationList": 0, "VTableShape": 0, "Size": 47}}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: predefined C++ types (compiler internal).rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===
