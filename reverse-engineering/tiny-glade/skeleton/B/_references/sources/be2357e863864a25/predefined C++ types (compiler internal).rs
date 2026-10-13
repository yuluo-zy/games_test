// Generated from PDB metadata by skeleton/_tools/fill_type_structures.py.
// Version: B
// 类型元数据；原作者源码文本/定义文件见逐项映射；不生成依赖实现。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: predefined C++ types (compiler internal).rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB forward_declaration_only] _s__RTTIBaseClassDescriptor; TPI 0x1bd6
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "_s__RTTIBaseClassDescriptor", "UniqueName": ".?AU_s__RTTIBaseClassDescriptor@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [7135]
// ------------------------------------------------------------------
// [PDB forward_declaration_only] _TypeDescriptor; TPI 0x1bd8
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "_TypeDescriptor", "UniqueName": ".?AU_TypeDescriptor@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [7188]
// ------------------------------------------------------------------
// [PDB forward_declaration_only] _PMD; TPI 0x1bda
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "_PMD", "UniqueName": ".?AU_PMD@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [7177]
// ------------------------------------------------------------------
// [PDB forward_declaration_only] _s__RTTIClassHierarchyDescriptor; TPI 0x1bdb
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "_s__RTTIClassHierarchyDescriptor", "UniqueName": ".?AU_s__RTTIClassHierarchyDescriptor@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [7154]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] _s__RTTIBaseClassDescriptor; TPI 0x1bdf
// 原始定义文件映射: PDB_UDT_source_line
// size = 36; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 5, "Options": ["None", "HasUniqueName"], "FieldList": 7134, "Name": "_s__RTTIBaseClassDescriptor", "UniqueName": ".?AU_s__RTTIBaseClassDescriptor@@", "DerivationList": 0, "VTableShape": 0, "Size": 36}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7134, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7129, "FieldOffset": 0, "Name": "pTypeDescriptor"}}, "type_references": [{"index": 7129, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 7128, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 7128, "kind": "LF_STRUCTURE", "spelling": "_TypeDescriptor", "status": "PDB_record_name"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7134, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 8, "Name": "numContainedBases"}}, "type_references": [{"index": 34, "spelling": "UInt32Long", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7134, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7130, "FieldOffset": 12, "Name": "where"}}, "type_references": [{"index": 7130, "kind": "LF_STRUCTURE", "spelling": "_PMD", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7134, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 24, "Name": "attributes"}}, "type_references": [{"index": 34, "spelling": "UInt32Long", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7134, "order": 4, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7133, "FieldOffset": 28, "Name": "pClassDescriptor"}}, "type_references": [{"index": 7133, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 7132, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 7132, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 7131, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 7131, "kind": "LF_STRUCTURE", "spelling": "_s__RTTIClassHierarchyDescriptor", "status": "PDB_record_name"}}}}}]}
// [PDB UDT 源码行] {"ipi_index": 125148, "original_path": "d:\\a01\\_work\\5\\s\\src\\vctools\\crt\\vcstartup\\build\\md\\msvcrt_kernel32\\predefined C++ types (compiler internal)", "line": 188, "module": null, "source_string_id": 125147}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] _s__RTTICompleteObjectLocator2; TPI 0x1be0
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "_s__RTTICompleteObjectLocator2", "UniqueName": ".?AU_s__RTTICompleteObjectLocator2@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [7140]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] _s__RTTICompleteObjectLocator2; TPI 0x1be4
// 原始定义文件映射: PDB_UDT_source_line
// size = 36; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 6, "Options": ["None", "HasUniqueName"], "FieldList": 7139, "Name": "_s__RTTICompleteObjectLocator2", "UniqueName": ".?AU_s__RTTICompleteObjectLocator2@@", "DerivationList": 0, "VTableShape": 0, "Size": 36}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7139, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 0, "Name": "signature"}}, "type_references": [{"index": 34, "spelling": "UInt32Long", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7139, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 4, "Name": "offset"}}, "type_references": [{"index": 34, "spelling": "UInt32Long", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7139, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 8, "Name": "cdOffset"}}, "type_references": [{"index": 34, "spelling": "UInt32Long", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7139, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7129, "FieldOffset": 12, "Name": "pTypeDescriptor"}}, "type_references": [{"index": 7129, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 7128, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 7128, "kind": "LF_STRUCTURE", "spelling": "_TypeDescriptor", "status": "PDB_record_name"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7139, "order": 4, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7133, "FieldOffset": 20, "Name": "pClassDescriptor"}}, "type_references": [{"index": 7133, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 7132, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 7132, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 7131, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 7131, "kind": "LF_STRUCTURE", "spelling": "_s__RTTIClassHierarchyDescriptor", "status": "PDB_record_name"}}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7139, "order": 5, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7138, "FieldOffset": 28, "Name": "pSelf"}}, "type_references": [{"index": 7138, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 7137, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 7137, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 7136, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 7136, "kind": "LF_STRUCTURE", "spelling": "_s__RTTICompleteObjectLocator2", "status": "PDB_record_name"}}}}}]}
// [PDB UDT 源码行] {"ipi_index": 125149, "original_path": "d:\\a01\\_work\\5\\s\\src\\vctools\\crt\\vcstartup\\build\\md\\msvcrt_kernel32\\predefined C++ types (compiler internal)", "line": 207, "module": null, "source_string_id": 125147}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] $_s__RTTIBaseClassArray$_extraBytes_24; TPI 0x1be5
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_s__RTTIBaseClassArray$_extraBytes_24", "UniqueName": ".?AU$_s__RTTIBaseClassArray$_extraBytes_24@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [7145]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] $_s__RTTIBaseClassArray$_extraBytes_24; TPI 0x1be9
// 原始定义文件映射: PDB_UDT_source_line
// size = 28; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 1, "Options": ["None", "HasUniqueName"], "FieldList": 7144, "Name": "$_s__RTTIBaseClassArray$_extraBytes_24", "UniqueName": ".?AU$_s__RTTIBaseClassArray$_extraBytes_24@@", "DerivationList": 0, "VTableShape": 0, "Size": 28}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7144, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7143, "FieldOffset": 0, "Name": "arrayOfBaseClassDescriptors"}}, "type_references": [{"index": 7143, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 7142, "IndexType": 35, "Size": 24, "Name": ""}}, "nested_types": {"ElementType": {"index": 7142, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 7127, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 7127, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 7126, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 7126, "kind": "LF_STRUCTURE", "spelling": "_s__RTTIBaseClassDescriptor", "status": "PDB_record_name"}}}}}}}]}
// [PDB UDT 源码行] {"ipi_index": 125150, "original_path": "d:\\a01\\_work\\5\\s\\src\\vctools\\crt\\vcstartup\\build\\md\\msvcrt_kernel32\\predefined C++ types (compiler internal)", "line": 197, "module": null, "source_string_id": 125147}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] $_TypeDescriptor$_extraBytes_20; TPI 0x1bea
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_TypeDescriptor$_extraBytes_20", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_20@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [7149]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] $_TypeDescriptor$_extraBytes_20; TPI 0x1bed
// 原始定义文件映射: PDB_UDT_source_line
// size = 36; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "HasUniqueName"], "FieldList": 7148, "Name": "$_TypeDescriptor$_extraBytes_20", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_20@@", "DerivationList": 0, "VTableShape": 0, "Size": 36}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7148, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4125, "FieldOffset": 0, "Name": "pVFTable"}}, "type_references": [{"index": 4125, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4124, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4124, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 3, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 3, "spelling": "Void", "status": "CodeView_simple_type; not Rust source spelling"}}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7148, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "spare"}}, "type_references": [{"index": 1539, "spelling": "Void / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7148, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7147, "FieldOffset": 16, "Name": "name"}}, "type_references": [{"index": 7147, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 112, "IndexType": 35, "Size": 20, "Name": ""}}, "nested_types": {"ElementType": {"index": 112, "spelling": "NarrowCharacter", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [PDB UDT 源码行] {"ipi_index": 125151, "original_path": "d:\\a01\\_work\\5\\s\\src\\vctools\\crt\\vcstartup\\build\\md\\msvcrt_kernel32\\predefined C++ types (compiler internal)", "line": 134, "module": null, "source_string_id": 125147}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] _s__RTTIBaseClassArray; TPI 0x1bee
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "_s__RTTIBaseClassArray", "UniqueName": ".?AU_s__RTTIBaseClassArray@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [7171]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] _s__RTTIClassHierarchyDescriptor; TPI 0x1bf2
// 原始定义文件映射: PDB_UDT_source_line
// size = 20; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None", "HasUniqueName"], "FieldList": 7153, "Name": "_s__RTTIClassHierarchyDescriptor", "UniqueName": ".?AU_s__RTTIClassHierarchyDescriptor@@", "DerivationList": 0, "VTableShape": 0, "Size": 20}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7153, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 0, "Name": "signature"}}, "type_references": [{"index": 34, "spelling": "UInt32Long", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7153, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 4, "Name": "attributes"}}, "type_references": [{"index": 34, "spelling": "UInt32Long", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7153, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 8, "Name": "numBaseClasses"}}, "type_references": [{"index": 34, "spelling": "UInt32Long", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7153, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7152, "FieldOffset": 12, "Name": "pBaseClassArray"}}, "type_references": [{"index": 7152, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 7151, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 7151, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 7150, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 7150, "kind": "LF_STRUCTURE", "spelling": "_s__RTTIBaseClassArray", "status": "PDB_record_name"}}}}}]}
// [PDB UDT 源码行] {"ipi_index": 125152, "original_path": "d:\\a01\\_work\\5\\s\\src\\vctools\\crt\\vcstartup\\build\\md\\msvcrt_kernel32\\predefined C++ types (compiler internal)", "line": 201, "module": null, "source_string_id": 125147}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] $_s__RTTIBaseClassArray$_extraBytes_16; TPI 0x1bf4
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_s__RTTIBaseClassArray$_extraBytes_16", "UniqueName": ".?AU$_s__RTTIBaseClassArray$_extraBytes_16@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [7159]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] $_s__RTTIBaseClassArray$_extraBytes_16; TPI 0x1bf7
// 原始定义文件映射: PDB_UDT_source_line
// size = 20; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 1, "Options": ["None", "HasUniqueName"], "FieldList": 7158, "Name": "$_s__RTTIBaseClassArray$_extraBytes_16", "UniqueName": ".?AU$_s__RTTIBaseClassArray$_extraBytes_16@@", "DerivationList": 0, "VTableShape": 0, "Size": 20}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7158, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7157, "FieldOffset": 0, "Name": "arrayOfBaseClassDescriptors"}}, "type_references": [{"index": 7157, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 7142, "IndexType": 35, "Size": 16, "Name": ""}}, "nested_types": {"ElementType": {"index": 7142, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 7127, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 7127, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 7126, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 7126, "kind": "LF_STRUCTURE", "spelling": "_s__RTTIBaseClassDescriptor", "status": "PDB_record_name"}}}}}}}]}
// [PDB UDT 源码行] {"ipi_index": 125153, "original_path": "d:\\a01\\_work\\5\\s\\src\\vctools\\crt\\vcstartup\\build\\md\\msvcrt_kernel32\\predefined C++ types (compiler internal)", "line": 197, "module": null, "source_string_id": 125147}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] $_TypeDescriptor$_extraBytes_24; TPI 0x1bf9
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_TypeDescriptor$_extraBytes_24", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_24@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [7164]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] $_TypeDescriptor$_extraBytes_24; TPI 0x1bfc
// 原始定义文件映射: PDB_UDT_source_line
// size = 40; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "HasUniqueName"], "FieldList": 7163, "Name": "$_TypeDescriptor$_extraBytes_24", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_24@@", "DerivationList": 0, "VTableShape": 0, "Size": 40}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7163, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4125, "FieldOffset": 0, "Name": "pVFTable"}}, "type_references": [{"index": 4125, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4124, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4124, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 3, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 3, "spelling": "Void", "status": "CodeView_simple_type; not Rust source spelling"}}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7163, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "spare"}}, "type_references": [{"index": 1539, "spelling": "Void / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7163, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7162, "FieldOffset": 16, "Name": "name"}}, "type_references": [{"index": 7162, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 112, "IndexType": 35, "Size": 24, "Name": ""}}, "nested_types": {"ElementType": {"index": 112, "spelling": "NarrowCharacter", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [PDB UDT 源码行] {"ipi_index": 125156, "original_path": "d:\\a01\\_work\\5\\s\\src\\vctools\\crt\\vcstartup\\build\\md\\msvcrt_kernel32\\predefined C++ types (compiler internal)", "line": 134, "module": null, "source_string_id": 125147}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] $_s__RTTIBaseClassArray$_extraBytes_8; TPI 0x1bfd
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_s__RTTIBaseClassArray$_extraBytes_8", "UniqueName": ".?AU$_s__RTTIBaseClassArray$_extraBytes_8@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [7168]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] $_s__RTTIBaseClassArray$_extraBytes_8; TPI 0x1c00
// 原始定义文件映射: PDB_UDT_source_line
// size = 12; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 1, "Options": ["None", "HasUniqueName"], "FieldList": 7167, "Name": "$_s__RTTIBaseClassArray$_extraBytes_8", "UniqueName": ".?AU$_s__RTTIBaseClassArray$_extraBytes_8@@", "DerivationList": 0, "VTableShape": 0, "Size": 12}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7167, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7166, "FieldOffset": 0, "Name": "arrayOfBaseClassDescriptors"}}, "type_references": [{"index": 7166, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 7142, "IndexType": 35, "Size": 8, "Name": ""}}, "nested_types": {"ElementType": {"index": 7142, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 7127, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 7127, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 7126, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 7126, "kind": "LF_STRUCTURE", "spelling": "_s__RTTIBaseClassDescriptor", "status": "PDB_record_name"}}}}}}}]}
// [PDB UDT 源码行] {"ipi_index": 125157, "original_path": "d:\\a01\\_work\\5\\s\\src\\vctools\\crt\\vcstartup\\build\\md\\msvcrt_kernel32\\predefined C++ types (compiler internal)", "line": 197, "module": null, "source_string_id": 125147}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] _s__RTTIBaseClassArray; TPI 0x1c03
// 原始定义文件映射: PDB_UDT_source_line
// size = 4; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 1, "Options": ["None", "HasUniqueName"], "FieldList": 7170, "Name": "_s__RTTIBaseClassArray", "UniqueName": ".?AU_s__RTTIBaseClassArray@@", "DerivationList": 0, "VTableShape": 0, "Size": 4}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7170, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7169, "FieldOffset": 0, "Name": "arrayOfBaseClassDescriptors"}}, "type_references": [{"index": 7169, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 7142, "IndexType": 35, "Size": 0, "Name": ""}}, "nested_types": {"ElementType": {"index": 7142, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 7127, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 7127, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 7126, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 7126, "kind": "LF_STRUCTURE", "spelling": "_s__RTTIBaseClassDescriptor", "status": "PDB_record_name"}}}}}}}]}
// [PDB UDT 源码行] {"ipi_index": 125159, "original_path": "d:\\a01\\_work\\5\\s\\src\\vctools\\crt\\vcstartup\\build\\md\\msvcrt_kernel32\\predefined C++ types (compiler internal)", "line": 197, "module": null, "source_string_id": 125147}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] $_TypeDescriptor$_extraBytes_21; TPI 0x1c04
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_TypeDescriptor$_extraBytes_21", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_21@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [7175]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] $_TypeDescriptor$_extraBytes_21; TPI 0x1c07
// 原始定义文件映射: PDB_UDT_source_line
// size = 37; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "HasUniqueName"], "FieldList": 7174, "Name": "$_TypeDescriptor$_extraBytes_21", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_21@@", "DerivationList": 0, "VTableShape": 0, "Size": 37}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7174, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4125, "FieldOffset": 0, "Name": "pVFTable"}}, "type_references": [{"index": 4125, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4124, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4124, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 3, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 3, "spelling": "Void", "status": "CodeView_simple_type; not Rust source spelling"}}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7174, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "spare"}}, "type_references": [{"index": 1539, "spelling": "Void / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7174, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7173, "FieldOffset": 16, "Name": "name"}}, "type_references": [{"index": 7173, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 112, "IndexType": 35, "Size": 21, "Name": ""}}, "nested_types": {"ElementType": {"index": 112, "spelling": "NarrowCharacter", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [PDB UDT 源码行] {"ipi_index": 125160, "original_path": "d:\\a01\\_work\\5\\s\\src\\vctools\\crt\\vcstartup\\build\\md\\msvcrt_kernel32\\predefined C++ types (compiler internal)", "line": 134, "module": null, "source_string_id": 125147}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] _PMD; TPI 0x1c09
// 原始定义文件映射: PDB_UDT_source_line
// size = 12; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "HasUniqueName"], "FieldList": 7176, "Name": "_PMD", "UniqueName": ".?AU_PMD@@", "DerivationList": 0, "VTableShape": 0, "Size": 12}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7176, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 116, "FieldOffset": 0, "Name": "mdisp"}}, "type_references": [{"index": 116, "spelling": "Int32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7176, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 116, "FieldOffset": 4, "Name": "pdisp"}}, "type_references": [{"index": 116, "spelling": "Int32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7176, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 116, "FieldOffset": 8, "Name": "vdisp"}}, "type_references": [{"index": 116, "spelling": "Int32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 125162, "original_path": "d:\\a01\\_work\\5\\s\\src\\vctools\\crt\\vcstartup\\build\\md\\msvcrt_kernel32\\predefined C++ types (compiler internal)", "line": 123, "module": null, "source_string_id": 125147}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] $_TypeDescriptor$_extraBytes_19; TPI 0x1c0a
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_TypeDescriptor$_extraBytes_19", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_19@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [7181]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] $_TypeDescriptor$_extraBytes_19; TPI 0x1c0d
// 原始定义文件映射: PDB_UDT_source_line
// size = 35; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "HasUniqueName"], "FieldList": 7180, "Name": "$_TypeDescriptor$_extraBytes_19", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_19@@", "DerivationList": 0, "VTableShape": 0, "Size": 35}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7180, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4125, "FieldOffset": 0, "Name": "pVFTable"}}, "type_references": [{"index": 4125, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4124, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4124, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 3, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 3, "spelling": "Void", "status": "CodeView_simple_type; not Rust source spelling"}}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7180, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "spare"}}, "type_references": [{"index": 1539, "spelling": "Void / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7180, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7179, "FieldOffset": 16, "Name": "name"}}, "type_references": [{"index": 7179, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 112, "IndexType": 35, "Size": 19, "Name": ""}}, "nested_types": {"ElementType": {"index": 112, "spelling": "NarrowCharacter", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [PDB UDT 源码行] {"ipi_index": 125163, "original_path": "d:\\a01\\_work\\5\\s\\src\\vctools\\crt\\vcstartup\\build\\md\\msvcrt_kernel32\\predefined C++ types (compiler internal)", "line": 134, "module": null, "source_string_id": 125147}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] $_TypeDescriptor$_extraBytes_28; TPI 0x1c0e
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_TypeDescriptor$_extraBytes_28", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_28@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [7185]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] $_TypeDescriptor$_extraBytes_28; TPI 0x1c11
// 原始定义文件映射: PDB_UDT_source_line
// size = 44; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "HasUniqueName"], "FieldList": 7184, "Name": "$_TypeDescriptor$_extraBytes_28", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_28@@", "DerivationList": 0, "VTableShape": 0, "Size": 44}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7184, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4125, "FieldOffset": 0, "Name": "pVFTable"}}, "type_references": [{"index": 4125, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4124, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4124, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 3, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 3, "spelling": "Void", "status": "CodeView_simple_type; not Rust source spelling"}}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7184, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "spare"}}, "type_references": [{"index": 1539, "spelling": "Void / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7184, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7183, "FieldOffset": 16, "Name": "name"}}, "type_references": [{"index": 7183, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 112, "IndexType": 35, "Size": 28, "Name": ""}}, "nested_types": {"ElementType": {"index": 112, "spelling": "NarrowCharacter", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [PDB UDT 源码行] {"ipi_index": 125174, "original_path": "d:\\a01\\_work\\5\\s\\src\\vctools\\crt\\vcstartup\\build\\md\\msvcrt_kernel32\\predefined C++ types (compiler internal)", "line": 134, "module": null, "source_string_id": 125147}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] _TypeDescriptor; TPI 0x1c14
// 原始定义文件映射: PDB_UDT_source_line
// size = 16; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "HasUniqueName"], "FieldList": 7187, "Name": "_TypeDescriptor", "UniqueName": ".?AU_TypeDescriptor@@", "DerivationList": 0, "VTableShape": 0, "Size": 16}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7187, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4125, "FieldOffset": 0, "Name": "pVFTable"}}, "type_references": [{"index": 4125, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4124, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4124, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 3, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 3, "spelling": "Void", "status": "CodeView_simple_type; not Rust source spelling"}}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7187, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "spare"}}, "type_references": [{"index": 1539, "spelling": "Void / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7187, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7186, "FieldOffset": 16, "Name": "name"}}, "type_references": [{"index": 7186, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 112, "IndexType": 35, "Size": 0, "Name": ""}}, "nested_types": {"ElementType": {"index": 112, "spelling": "NarrowCharacter", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [PDB UDT 源码行] {"ipi_index": 125175, "original_path": "d:\\a01\\_work\\5\\s\\src\\vctools\\crt\\vcstartup\\build\\md\\msvcrt_kernel32\\predefined C++ types (compiler internal)", "line": 134, "module": null, "source_string_id": 125147}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] $_TypeDescriptor$_extraBytes_16; TPI 0x1c15
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_TypeDescriptor$_extraBytes_16", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_16@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [7192]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] $_TypeDescriptor$_extraBytes_16; TPI 0x1c18
// 原始定义文件映射: PDB_UDT_source_line
// size = 32; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "HasUniqueName"], "FieldList": 7191, "Name": "$_TypeDescriptor$_extraBytes_16", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_16@@", "DerivationList": 0, "VTableShape": 0, "Size": 32}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7191, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4125, "FieldOffset": 0, "Name": "pVFTable"}}, "type_references": [{"index": 4125, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4124, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4124, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 3, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 3, "spelling": "Void", "status": "CodeView_simple_type; not Rust source spelling"}}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7191, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "spare"}}, "type_references": [{"index": 1539, "spelling": "Void / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7191, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7190, "FieldOffset": 16, "Name": "name"}}, "type_references": [{"index": 7190, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 112, "IndexType": 35, "Size": 16, "Name": ""}}, "nested_types": {"ElementType": {"index": 112, "spelling": "NarrowCharacter", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [PDB UDT 源码行] {"ipi_index": 125176, "original_path": "d:\\a01\\_work\\5\\s\\src\\vctools\\crt\\vcstartup\\build\\md\\msvcrt_kernel32\\predefined C++ types (compiler internal)", "line": 134, "module": null, "source_string_id": 125147}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] $_TypeDescriptor$_extraBytes_31; TPI 0x1c19
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_TypeDescriptor$_extraBytes_31", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_31@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [7196]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] $_TypeDescriptor$_extraBytes_31; TPI 0x1c1c
// 原始定义文件映射: PDB_UDT_source_line
// size = 47; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "HasUniqueName"], "FieldList": 7195, "Name": "$_TypeDescriptor$_extraBytes_31", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_31@@", "DerivationList": 0, "VTableShape": 0, "Size": 47}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7195, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4125, "FieldOffset": 0, "Name": "pVFTable"}}, "type_references": [{"index": 4125, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4124, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4124, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 3, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 3, "spelling": "Void", "status": "CodeView_simple_type; not Rust source spelling"}}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7195, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "spare"}}, "type_references": [{"index": 1539, "spelling": "Void / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7195, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7194, "FieldOffset": 16, "Name": "name"}}, "type_references": [{"index": 7194, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 112, "IndexType": 35, "Size": 31, "Name": ""}}, "nested_types": {"ElementType": {"index": 112, "spelling": "NarrowCharacter", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [PDB UDT 源码行] {"ipi_index": 125177, "original_path": "d:\\a01\\_work\\5\\s\\src\\vctools\\crt\\vcstartup\\build\\md\\msvcrt_kernel32\\predefined C++ types (compiler internal)", "line": 134, "module": null, "source_string_id": 125147}
// === 完整 TPI 结构关系引用记录；原文不转换成猜测的 Rust ABI ===
// [TPI 0x101c] {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 3, "Modifiers": ["None", "Const"]}}
// [TPI 0x101d] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4124, "Attrs": 65548}}
// [TPI 0x1bd6] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "_s__RTTIBaseClassDescriptor", "UniqueName": ".?AU_s__RTTIBaseClassDescriptor@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1bd7] {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 7126, "Modifiers": ["None", "Const"]}}
// [TPI 0x1bd8] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "_TypeDescriptor", "UniqueName": ".?AU_TypeDescriptor@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1bd9] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 7128, "Attrs": 65548}}
// [TPI 0x1bda] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "_PMD", "UniqueName": ".?AU_PMD@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1bdb] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "_s__RTTIClassHierarchyDescriptor", "UniqueName": ".?AU_s__RTTIClassHierarchyDescriptor@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1bdc] {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 7131, "Modifiers": ["None", "Const"]}}
// [TPI 0x1bdd] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 7132, "Attrs": 65548}}
// [TPI 0x1bde] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7129, "FieldOffset": 0, "Name": "pTypeDescriptor"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 8, "Name": "numContainedBases"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7130, "FieldOffset": 12, "Name": "where"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 24, "Name": "attributes"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7133, "FieldOffset": 28, "Name": "pClassDescriptor"}}]}
// [TPI 0x1bdf] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 5, "Options": ["None", "HasUniqueName"], "FieldList": 7134, "Name": "_s__RTTIBaseClassDescriptor", "UniqueName": ".?AU_s__RTTIBaseClassDescriptor@@", "DerivationList": 0, "VTableShape": 0, "Size": 36}}
// [TPI 0x1be0] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "_s__RTTICompleteObjectLocator2", "UniqueName": ".?AU_s__RTTICompleteObjectLocator2@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1be1] {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 7136, "Modifiers": ["None", "Const"]}}
// [TPI 0x1be2] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 7137, "Attrs": 65548}}
// [TPI 0x1be3] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 0, "Name": "signature"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 4, "Name": "offset"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 8, "Name": "cdOffset"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7129, "FieldOffset": 12, "Name": "pTypeDescriptor"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7133, "FieldOffset": 20, "Name": "pClassDescriptor"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7138, "FieldOffset": 28, "Name": "pSelf"}}]}
// [TPI 0x1be4] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 6, "Options": ["None", "HasUniqueName"], "FieldList": 7139, "Name": "_s__RTTICompleteObjectLocator2", "UniqueName": ".?AU_s__RTTICompleteObjectLocator2@@", "DerivationList": 0, "VTableShape": 0, "Size": 36}}
// [TPI 0x1be5] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_s__RTTIBaseClassArray$_extraBytes_24", "UniqueName": ".?AU$_s__RTTIBaseClassArray$_extraBytes_24@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1be6] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 7127, "Attrs": 65548}}
// [TPI 0x1be7] {"Kind": "LF_ARRAY", "Array": {"ElementType": 7142, "IndexType": 35, "Size": 24, "Name": ""}}
// [TPI 0x1be8] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7143, "FieldOffset": 0, "Name": "arrayOfBaseClassDescriptors"}}]}
// [TPI 0x1be9] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 1, "Options": ["None", "HasUniqueName"], "FieldList": 7144, "Name": "$_s__RTTIBaseClassArray$_extraBytes_24", "UniqueName": ".?AU$_s__RTTIBaseClassArray$_extraBytes_24@@", "DerivationList": 0, "VTableShape": 0, "Size": 28}}
// [TPI 0x1bea] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_TypeDescriptor$_extraBytes_20", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_20@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1beb] {"Kind": "LF_ARRAY", "Array": {"ElementType": 112, "IndexType": 35, "Size": 20, "Name": ""}}
// [TPI 0x1bec] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4125, "FieldOffset": 0, "Name": "pVFTable"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "spare"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7147, "FieldOffset": 16, "Name": "name"}}]}
// [TPI 0x1bed] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "HasUniqueName"], "FieldList": 7148, "Name": "$_TypeDescriptor$_extraBytes_20", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_20@@", "DerivationList": 0, "VTableShape": 0, "Size": 36}}
// [TPI 0x1bee] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "_s__RTTIBaseClassArray", "UniqueName": ".?AU_s__RTTIBaseClassArray@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1bef] {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 7150, "Modifiers": ["None", "Const"]}}
// [TPI 0x1bf0] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 7151, "Attrs": 65548}}
// [TPI 0x1bf1] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 0, "Name": "signature"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 4, "Name": "attributes"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 8, "Name": "numBaseClasses"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7152, "FieldOffset": 12, "Name": "pBaseClassArray"}}]}
// [TPI 0x1bf2] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None", "HasUniqueName"], "FieldList": 7153, "Name": "_s__RTTIClassHierarchyDescriptor", "UniqueName": ".?AU_s__RTTIClassHierarchyDescriptor@@", "DerivationList": 0, "VTableShape": 0, "Size": 20}}
// [TPI 0x1bf4] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_s__RTTIBaseClassArray$_extraBytes_16", "UniqueName": ".?AU$_s__RTTIBaseClassArray$_extraBytes_16@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1bf5] {"Kind": "LF_ARRAY", "Array": {"ElementType": 7142, "IndexType": 35, "Size": 16, "Name": ""}}
// [TPI 0x1bf6] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7157, "FieldOffset": 0, "Name": "arrayOfBaseClassDescriptors"}}]}
// [TPI 0x1bf7] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 1, "Options": ["None", "HasUniqueName"], "FieldList": 7158, "Name": "$_s__RTTIBaseClassArray$_extraBytes_16", "UniqueName": ".?AU$_s__RTTIBaseClassArray$_extraBytes_16@@", "DerivationList": 0, "VTableShape": 0, "Size": 20}}
// [TPI 0x1bf9] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_TypeDescriptor$_extraBytes_24", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_24@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1bfa] {"Kind": "LF_ARRAY", "Array": {"ElementType": 112, "IndexType": 35, "Size": 24, "Name": ""}}
// [TPI 0x1bfb] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4125, "FieldOffset": 0, "Name": "pVFTable"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "spare"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7162, "FieldOffset": 16, "Name": "name"}}]}
// [TPI 0x1bfc] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "HasUniqueName"], "FieldList": 7163, "Name": "$_TypeDescriptor$_extraBytes_24", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_24@@", "DerivationList": 0, "VTableShape": 0, "Size": 40}}
// [TPI 0x1bfd] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_s__RTTIBaseClassArray$_extraBytes_8", "UniqueName": ".?AU$_s__RTTIBaseClassArray$_extraBytes_8@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1bfe] {"Kind": "LF_ARRAY", "Array": {"ElementType": 7142, "IndexType": 35, "Size": 8, "Name": ""}}
// [TPI 0x1bff] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7166, "FieldOffset": 0, "Name": "arrayOfBaseClassDescriptors"}}]}
// [TPI 0x1c00] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 1, "Options": ["None", "HasUniqueName"], "FieldList": 7167, "Name": "$_s__RTTIBaseClassArray$_extraBytes_8", "UniqueName": ".?AU$_s__RTTIBaseClassArray$_extraBytes_8@@", "DerivationList": 0, "VTableShape": 0, "Size": 12}}
// [TPI 0x1c01] {"Kind": "LF_ARRAY", "Array": {"ElementType": 7142, "IndexType": 35, "Size": 0, "Name": ""}}
// [TPI 0x1c02] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7169, "FieldOffset": 0, "Name": "arrayOfBaseClassDescriptors"}}]}
// [TPI 0x1c03] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 1, "Options": ["None", "HasUniqueName"], "FieldList": 7170, "Name": "_s__RTTIBaseClassArray", "UniqueName": ".?AU_s__RTTIBaseClassArray@@", "DerivationList": 0, "VTableShape": 0, "Size": 4}}
// [TPI 0x1c04] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_TypeDescriptor$_extraBytes_21", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_21@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1c05] {"Kind": "LF_ARRAY", "Array": {"ElementType": 112, "IndexType": 35, "Size": 21, "Name": ""}}
// [TPI 0x1c06] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4125, "FieldOffset": 0, "Name": "pVFTable"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "spare"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7173, "FieldOffset": 16, "Name": "name"}}]}
// [TPI 0x1c07] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "HasUniqueName"], "FieldList": 7174, "Name": "$_TypeDescriptor$_extraBytes_21", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_21@@", "DerivationList": 0, "VTableShape": 0, "Size": 37}}
// [TPI 0x1c08] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 116, "FieldOffset": 0, "Name": "mdisp"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 116, "FieldOffset": 4, "Name": "pdisp"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 116, "FieldOffset": 8, "Name": "vdisp"}}]}
// [TPI 0x1c09] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "HasUniqueName"], "FieldList": 7176, "Name": "_PMD", "UniqueName": ".?AU_PMD@@", "DerivationList": 0, "VTableShape": 0, "Size": 12}}
// [TPI 0x1c0a] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_TypeDescriptor$_extraBytes_19", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_19@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1c0b] {"Kind": "LF_ARRAY", "Array": {"ElementType": 112, "IndexType": 35, "Size": 19, "Name": ""}}
// [TPI 0x1c0c] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4125, "FieldOffset": 0, "Name": "pVFTable"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "spare"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7179, "FieldOffset": 16, "Name": "name"}}]}
// [TPI 0x1c0d] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "HasUniqueName"], "FieldList": 7180, "Name": "$_TypeDescriptor$_extraBytes_19", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_19@@", "DerivationList": 0, "VTableShape": 0, "Size": 35}}
// [TPI 0x1c0e] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_TypeDescriptor$_extraBytes_28", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_28@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1c0f] {"Kind": "LF_ARRAY", "Array": {"ElementType": 112, "IndexType": 35, "Size": 28, "Name": ""}}
// [TPI 0x1c10] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4125, "FieldOffset": 0, "Name": "pVFTable"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "spare"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7183, "FieldOffset": 16, "Name": "name"}}]}
// [TPI 0x1c11] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "HasUniqueName"], "FieldList": 7184, "Name": "$_TypeDescriptor$_extraBytes_28", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_28@@", "DerivationList": 0, "VTableShape": 0, "Size": 44}}
// [TPI 0x1c12] {"Kind": "LF_ARRAY", "Array": {"ElementType": 112, "IndexType": 35, "Size": 0, "Name": ""}}
// [TPI 0x1c13] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4125, "FieldOffset": 0, "Name": "pVFTable"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "spare"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7186, "FieldOffset": 16, "Name": "name"}}]}
// [TPI 0x1c14] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "HasUniqueName"], "FieldList": 7187, "Name": "_TypeDescriptor", "UniqueName": ".?AU_TypeDescriptor@@", "DerivationList": 0, "VTableShape": 0, "Size": 16}}
// [TPI 0x1c15] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_TypeDescriptor$_extraBytes_16", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_16@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1c16] {"Kind": "LF_ARRAY", "Array": {"ElementType": 112, "IndexType": 35, "Size": 16, "Name": ""}}
// [TPI 0x1c17] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4125, "FieldOffset": 0, "Name": "pVFTable"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "spare"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7190, "FieldOffset": 16, "Name": "name"}}]}
// [TPI 0x1c18] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "HasUniqueName"], "FieldList": 7191, "Name": "$_TypeDescriptor$_extraBytes_16", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_16@@", "DerivationList": 0, "VTableShape": 0, "Size": 32}}
// [TPI 0x1c19] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "$_TypeDescriptor$_extraBytes_31", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_31@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1c1a] {"Kind": "LF_ARRAY", "Array": {"ElementType": 112, "IndexType": 35, "Size": 31, "Name": ""}}
// [TPI 0x1c1b] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4125, "FieldOffset": 0, "Name": "pVFTable"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "spare"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7194, "FieldOffset": 16, "Name": "name"}}]}
// [TPI 0x1c1c] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "HasUniqueName"], "FieldList": 7195, "Name": "$_TypeDescriptor$_extraBytes_31", "UniqueName": ".?AU$_TypeDescriptor$_extraBytes_31@@", "DerivationList": 0, "VTableShape": 0, "Size": 47}}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: predefined C++ types (compiler internal).rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===
