// Generated from PDB metadata by skeleton/_tools/fill_type_structures.py.
// Version: A
// 类型元数据；原作者源码文本/定义文件见逐项映射；不生成依赖实现。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: lcms2_plugin.h.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB forward_declaration_only] _cms_io_handler; TPI 0x1272
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cms_io_handler", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [4757]
// ------------------------------------------------------------------
// [PDB forward_declaration_only] _cms_typehandler_struct; TPI 0x1280
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cms_typehandler_struct", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [4786]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] _cms_io_handler; TPI 0x1295
// 原始定义文件映射: PDB_UDT_source_line
// size = 320; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 10, "Options": ["None"], "FieldList": 4756, "Name": "_cms_io_handler", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 320}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4756, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 0, "Name": "stream"}}, "type_references": [{"index": 1539, "spelling": "Void / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4756, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4725, "FieldOffset": 8, "Name": "ContextID"}}, "type_references": [{"index": 4725, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4724, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4724, "kind": "LF_STRUCTURE", "spelling": "_cmsContext_struct", "status": "PDB_record_name"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4756, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 16, "Name": "UsedSpace"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4756, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 20, "Name": "ReportedSize"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4756, "order": 4, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4741, "FieldOffset": 24, "Name": "PhysicalFile"}}, "type_references": [{"index": 4741, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 112, "IndexType": 35, "Size": 256, "Name": ""}}, "nested_types": {"ElementType": {"index": 112, "spelling": "NarrowCharacter", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4756, "order": 5, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4744, "FieldOffset": 280, "Name": "Read"}}, "type_references": [{"index": 4744, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4743, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4743, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 117, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 4, "ArgumentList": 4742}}, "nested_types": {}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4756, "order": 6, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4747, "FieldOffset": 288, "Name": "Seek"}}, "type_references": [{"index": 4747, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4746, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4746, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 116, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 2, "ArgumentList": 4745}}, "nested_types": {}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4756, "order": 7, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4750, "FieldOffset": 296, "Name": "Close"}}, "type_references": [{"index": 4750, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4749, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4749, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 116, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 1, "ArgumentList": 4748}}, "nested_types": {}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4756, "order": 8, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4752, "FieldOffset": 304, "Name": "Tell"}}, "type_references": [{"index": 4752, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4751, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4751, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 117, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 1, "ArgumentList": 4748}}, "nested_types": {}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4756, "order": 9, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4755, "FieldOffset": 312, "Name": "Write"}}, "type_references": [{"index": 4755, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4754, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4754, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 116, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 3, "ArgumentList": 4753}}, "nested_types": {}}}}]}
// [PDB UDT 源码行] {"ipi_index": 66301, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lcms2-sys-4.0.1/vendor/include/lcms2_plugin.h", "line": 118, "module": null, "source_string_id": 66300}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] _cms_typehandler_struct; TPI 0x12b2
// 原始定义文件映射: PDB_UDT_source_line
// size = 56; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 7, "Options": ["None"], "FieldList": 4785, "Name": "_cms_typehandler_struct", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 56}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4785, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4772, "FieldOffset": 0, "Name": "Signature"}}, "type_references": [{"index": 4772, "kind": "LF_ENUM", "spelling": "<unnamed-tag>", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4785, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4775, "FieldOffset": 8, "Name": "ReadPtr"}}, "type_references": [{"index": 4775, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4774, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4774, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 1539, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 4, "ArgumentList": 4773}}, "nested_types": {}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4785, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4778, "FieldOffset": 16, "Name": "WritePtr"}}, "type_references": [{"index": 4778, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4777, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4777, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 116, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 4, "ArgumentList": 4776}}, "nested_types": {}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4785, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4781, "FieldOffset": 24, "Name": "DupPtr"}}, "type_references": [{"index": 4781, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4780, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4780, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 1539, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 3, "ArgumentList": 4779}}, "nested_types": {}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4785, "order": 4, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4784, "FieldOffset": 32, "Name": "FreePtr"}}, "type_references": [{"index": 4784, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4783, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4783, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 3, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 2, "ArgumentList": 4782}}, "nested_types": {}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4785, "order": 5, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4725, "FieldOffset": 40, "Name": "ContextID"}}, "type_references": [{"index": 4725, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4724, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4724, "kind": "LF_STRUCTURE", "spelling": "_cmsContext_struct", "status": "PDB_record_name"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4785, "order": 6, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 48, "Name": "ICCVersion"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 66307, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lcms2-sys-4.0.1/vendor/include/lcms2_plugin.h", "line": 387, "module": null, "source_string_id": 66300}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] cmsTagDescriptor; TPI 0x12cf
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsTagDescriptor", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [4825]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] cmsTagDescriptor; TPI 0x12d9
// 原始定义文件映射: PDB_UDT_source_line
// size = 96; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 4824, "Name": "cmsTagDescriptor", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 96}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4824, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 0, "Name": "ElemCount"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4824, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 4, "Name": "nSupportedTypes"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4824, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4820, "FieldOffset": 8, "Name": "SupportedTypes"}}, "type_references": [{"index": 4820, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 4772, "IndexType": 35, "Size": 80, "Name": ""}}, "nested_types": {"ElementType": {"index": 4772, "kind": "LF_ENUM", "spelling": "<unnamed-tag>", "status": "PDB_record_name"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4824, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4823, "FieldOffset": 88, "Name": "DecideType"}}, "type_references": [{"index": 4823, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4822, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4822, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 4772, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 2, "ArgumentList": 4821}}, "nested_types": {}}}}]}
// [PDB UDT 源码行] {"ipi_index": 66315, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lcms2-sys-4.0.1/vendor/include/lcms2_plugin.h", "line": 430, "module": null, "source_string_id": 66300}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] _cms_interp_struc; TPI 0x1350
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cms_interp_struc", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [4960]
// ------------------------------------------------------------------
// [PDB forward_declaration_only] cmsInterpFunction; TPI 0x135e
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_UNION", "Union": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsInterpFunction", "UniqueName": "", "Size": 0}}
// 前向定义候选（仅名称关联）: [4975]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] _cms_interp_struc; TPI 0x1360
// 原始定义文件映射: PDB_UDT_source_line
// size = 216; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 9, "Options": ["None"], "FieldList": 4959, "Name": "_cms_interp_struc", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 216}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4959, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4725, "FieldOffset": 0, "Name": "ContextID"}}, "type_references": [{"index": 4725, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4724, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4724, "kind": "LF_STRUCTURE", "spelling": "_cmsContext_struct", "status": "PDB_record_name"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4959, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 8, "Name": "dwFlags"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4959, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 12, "Name": "nInputs"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4959, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 16, "Name": "nOutputs"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4959, "order": 4, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4957, "FieldOffset": 20, "Name": "nSamples"}}, "type_references": [{"index": 4957, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 117, "IndexType": 35, "Size": 60, "Name": ""}}, "nested_types": {"ElementType": {"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4959, "order": 5, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4957, "FieldOffset": 80, "Name": "Domain"}}, "type_references": [{"index": 4957, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 117, "IndexType": 35, "Size": 60, "Name": ""}}, "nested_types": {"ElementType": {"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4959, "order": 6, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4957, "FieldOffset": 140, "Name": "opta"}}, "type_references": [{"index": 4957, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 117, "IndexType": 35, "Size": 60, "Name": ""}}, "nested_types": {"ElementType": {"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4959, "order": 7, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4353, "FieldOffset": 200, "Name": "Table"}}, "type_references": [{"index": 4353, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4352, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4352, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 3, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 3, "spelling": "Void", "status": "CodeView_simple_type; not Rust source spelling"}}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4959, "order": 8, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4958, "FieldOffset": 208, "Name": "Interpolation"}}, "type_references": [{"index": 4958, "kind": "LF_UNION", "spelling": "cmsInterpFunction", "status": "PDB_record_name"}]}
// [PDB UDT 源码行] {"ipi_index": 66415, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lcms2-sys-4.0.1/vendor/include/lcms2_plugin.h", "line": 292, "module": null, "source_string_id": 66300}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] cmsInterpFunction; TPI 0x136f
// 原始定义文件映射: PDB_UDT_source_line
// size = 8; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_UNION", "Union": {"MemberCount": 2, "Options": ["None", "Sealed"], "FieldList": 4974, "Name": "cmsInterpFunction", "UniqueName": "", "Size": 8}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4974, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4968, "FieldOffset": 0, "Name": "Lerp16"}}, "type_references": [{"index": 4968, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4967, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4967, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 3, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 3, "ArgumentList": 4966}}, "nested_types": {}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4974, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4973, "FieldOffset": 0, "Name": "LerpFloat"}}, "type_references": [{"index": 4973, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4972, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4972, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 3, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 3, "ArgumentList": 4971}}, "nested_types": {}}}}]}
// [PDB UDT 源码行] {"ipi_index": 66417, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lcms2-sys-4.0.1/vendor/include/lcms2_plugin.h", "line": 279, "module": null, "source_string_id": 66300}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] cmsMAT3; TPI 0x13a9
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsMAT3", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [5037]
// ------------------------------------------------------------------
// [PDB forward_declaration_only] cmsVEC3; TPI 0x13aa
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsVEC3", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [5040]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] cmsMAT3; TPI 0x13ad
// 原始定义文件映射: PDB_UDT_source_line
// size = 72; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 1, "Options": ["None"], "FieldList": 5036, "Name": "cmsMAT3", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 72}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5036, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5035, "FieldOffset": 0, "Name": "v"}}, "type_references": [{"index": 5035, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 5034, "IndexType": 35, "Size": 72, "Name": ""}}, "nested_types": {"ElementType": {"index": 5034, "kind": "LF_STRUCTURE", "spelling": "cmsVEC3", "status": "PDB_record_name"}}}]}
// [PDB UDT 源码行] {"ipi_index": 66431, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lcms2-sys-4.0.1/vendor/include/lcms2_plugin.h", "line": 77, "module": null, "source_string_id": 66300}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] cmsVEC3; TPI 0x13b0
// 原始定义文件映射: PDB_UDT_source_line
// size = 24; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 1, "Options": ["None"], "FieldList": 5039, "Name": "cmsVEC3", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 24}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5039, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5038, "FieldOffset": 0, "Name": "n"}}, "type_references": [{"index": 5038, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 65, "IndexType": 35, "Size": 24, "Name": ""}}, "nested_types": {"ElementType": {"index": 65, "spelling": "Float64", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [PDB UDT 源码行] {"ipi_index": 66432, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lcms2-sys-4.0.1/vendor/include/lcms2_plugin.h", "line": 71, "module": null, "source_string_id": 66300}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] cmsStride; TPI 0x13fb
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsStride", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [5136]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] cmsStride; TPI 0x1410
// 原始定义文件映射: PDB_UDT_source_line
// size = 16; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 5135, "Name": "cmsStride", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 16}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5135, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 0, "Name": "BytesPerLineIn"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5135, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 4, "Name": "BytesPerLineOut"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5135, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 8, "Name": "BytesPerPlaneIn"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5135, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 12, "Name": "BytesPerPlaneOut"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 66469, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lcms2-sys-4.0.1/vendor/include/lcms2_plugin.h", "line": 588, "module": null, "source_string_id": 66300}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] _cmsPluginBaseStruct; TPI 0x1458
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cmsPluginBaseStruct", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [5213]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] _cmsPluginBaseStruct; TPI 0x145d
// 原始定义文件映射: PDB_UDT_source_line
// size = 24; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 5212, "Name": "_cmsPluginBaseStruct", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 24}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5212, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 0, "Name": "Magic"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5212, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 4, "Name": "ExpectedVersion"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5212, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 8, "Name": "Type"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5212, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5209, "FieldOffset": 16, "Name": "Next"}}, "type_references": [{"index": 5209, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 5208, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 5208, "kind": "LF_STRUCTURE", "spelling": "_cmsPluginBaseStruct", "status": "PDB_record_name"}}}]}
// [PDB UDT 源码行] {"ipi_index": 66502, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lcms2-sys-4.0.1/vendor/include/lcms2_plugin.h", "line": 214, "module": null, "source_string_id": 66300}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] cmsPluginTransform; TPI 0x145e
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsPluginTransform", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [5223]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] cmsPluginTransform::<unnamed-tag>; TPI 0x1465
// 原始定义文件映射: PDB_UDT_source_line
// size = 8; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_UNION", "Union": {"MemberCount": 2, "Options": ["None", "Nested", "Sealed"], "FieldList": 5220, "Name": "cmsPluginTransform::<unnamed-tag>", "UniqueName": "", "Size": 8}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5220, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5219, "FieldOffset": 0, "Name": "legacy_xform"}}, "type_references": [{"index": 5219, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 5218, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 5218, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 116, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 7, "ArgumentList": 5217}}, "nested_types": {}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5220, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5204, "FieldOffset": 0, "Name": "xform"}}, "type_references": [{"index": 5204, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 5203, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 5203, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 116, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 7, "ArgumentList": 5202}}, "nested_types": {}}}}]}
// [PDB UDT 源码行] {"ipi_index": 66504, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lcms2-sys-4.0.1/vendor/include/lcms2_plugin.h", "line": 643, "module": null, "source_string_id": 66300}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] cmsPluginTransform; TPI 0x1467
// 原始定义文件映射: PDB_UDT_source_line
// size = 32; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "ContainsNestedClass"], "FieldList": 5222, "Name": "cmsPluginTransform", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 32}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5222, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5208, "FieldOffset": 0, "Name": "base"}}, "type_references": [{"index": 5208, "kind": "LF_STRUCTURE", "spelling": "_cmsPluginBaseStruct", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5222, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5221, "FieldOffset": 24, "Name": "factories"}}, "type_references": [{"index": 5221, "kind": "LF_UNION", "spelling": "cmsPluginTransform::<unnamed-tag>", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5222, "order": 2, "kind": "LF_NESTTYPE", "record": {"Kind": "LF_NESTTYPE", "NestedType": {"Type": 5221, "Name": ""}}, "type_references": [{"index": 5221, "kind": "LF_UNION", "spelling": "cmsPluginTransform::<unnamed-tag>", "status": "PDB_record_name"}]}
// [PDB UDT 源码行] {"ipi_index": 66505, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lcms2-sys-4.0.1/vendor/include/lcms2_plugin.h", "line": 639, "module": null, "source_string_id": 66300}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] cmsPluginMemHandler; TPI 0x14a6
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsPluginMemHandler", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [5292]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] cmsPluginMemHandler; TPI 0x14ac
// 原始定义文件映射: PDB_UDT_source_line
// size = 72; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 7, "Options": ["None"], "FieldList": 5291, "Name": "cmsPluginMemHandler", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 72}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5291, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5208, "FieldOffset": 0, "Name": "base"}}, "type_references": [{"index": 5208, "kind": "LF_STRUCTURE", "spelling": "_cmsPluginBaseStruct", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5291, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4793, "FieldOffset": 24, "Name": "MallocPtr"}}, "type_references": [{"index": 4793, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4792, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4792, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 1539, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 2, "ArgumentList": 4791}}, "nested_types": {}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5291, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4796, "FieldOffset": 32, "Name": "FreePtr"}}, "type_references": [{"index": 4796, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4795, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4795, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 3, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 2, "ArgumentList": 4794}}, "nested_types": {}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5291, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4799, "FieldOffset": 40, "Name": "ReallocPtr"}}, "type_references": [{"index": 4799, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4798, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4798, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 1539, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 3, "ArgumentList": 4797}}, "nested_types": {}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5291, "order": 4, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4793, "FieldOffset": 48, "Name": "MallocZeroPtr"}}, "type_references": [{"index": 4793, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4792, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4792, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 1539, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 2, "ArgumentList": 4791}}, "nested_types": {}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5291, "order": 5, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4802, "FieldOffset": 56, "Name": "CallocPtr"}}, "type_references": [{"index": 4802, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4801, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4801, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 1539, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 3, "ArgumentList": 4800}}, "nested_types": {}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5291, "order": 6, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4805, "FieldOffset": 64, "Name": "DupPtr"}}, "type_references": [{"index": 4805, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4804, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4804, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 1539, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 3, "ArgumentList": 4803}}, "nested_types": {}}}}]}
// [PDB UDT 源码行] {"ipi_index": 93971, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lcms2-sys-4.0.1/vendor/include/lcms2_plugin.h", "line": 238, "module": null, "source_string_id": 66300}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] cmsPluginMutex; TPI 0x14e8
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsPluginMutex", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [5355]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] cmsPluginMutex; TPI 0x14eb
// 原始定义文件映射: PDB_UDT_source_line
// size = 56; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 5, "Options": ["None"], "FieldList": 5354, "Name": "cmsPluginMutex", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 56}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5354, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5208, "FieldOffset": 0, "Name": "base"}}, "type_references": [{"index": 5208, "kind": "LF_STRUCTURE", "spelling": "_cmsPluginBaseStruct", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5354, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5348, "FieldOffset": 24, "Name": "CreateMutexPtr"}}, "type_references": [{"index": 5348, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4829, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4829, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 1539, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 1, "ArgumentList": 4828}}, "nested_types": {}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5354, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4796, "FieldOffset": 32, "Name": "DestroyMutexPtr"}}, "type_references": [{"index": 4796, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4795, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4795, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 3, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 2, "ArgumentList": 4794}}, "nested_types": {}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5354, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5349, "FieldOffset": 40, "Name": "LockMutexPtr"}}, "type_references": [{"index": 5349, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 5346, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 5346, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 116, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 2, "ArgumentList": 4794}}, "nested_types": {}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5354, "order": 4, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4796, "FieldOffset": 48, "Name": "UnlockMutexPtr"}}, "type_references": [{"index": 4796, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4795, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4795, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 3, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 2, "ArgumentList": 4794}}, "nested_types": {}}}}]}
// [PDB UDT 源码行] {"ipi_index": 94018, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lcms2-sys-4.0.1/vendor/include/lcms2_plugin.h", "line": 658, "module": null, "source_string_id": 66300}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] cmsPluginParalellization; TPI 0x14ed
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsPluginParalellization", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [5360]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] cmsPluginParalellization; TPI 0x14f0
// 原始定义文件映射: PDB_UDT_source_line
// size = 40; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 5359, "Name": "cmsPluginParalellization", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 40}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5359, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5208, "FieldOffset": 0, "Name": "base"}}, "type_references": [{"index": 5208, "kind": "LF_STRUCTURE", "spelling": "_cmsPluginBaseStruct", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5359, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 116, "FieldOffset": 24, "Name": "MaxWorkers"}}, "type_references": [{"index": 116, "spelling": "Int32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5359, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 28, "Name": "WorkerFlags"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5359, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5120, "FieldOffset": 32, "Name": "SchedulerFn"}}, "type_references": [{"index": 5120, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 5119, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 5119, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 3, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 6, "ArgumentList": 5118}}, "nested_types": {}}}}]}
// [PDB UDT 源码行] {"ipi_index": 94025, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lcms2-sys-4.0.1/vendor/include/lcms2_plugin.h", "line": 683, "module": null, "source_string_id": 66300}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] _cmsTagBase; TPI 0x1559
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cmsTagBase", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [5468]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] _cmsTagBase; TPI 0x155c
// 原始定义文件映射: PDB_UDT_source_line
// size = 8; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 5467, "Name": "_cmsTagBase", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 8}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5467, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4772, "FieldOffset": 0, "Name": "sig"}}, "type_references": [{"index": 4772, "kind": "LF_ENUM", "spelling": "<unnamed-tag>", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5467, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5466, "FieldOffset": 4, "Name": "reserved"}}, "type_references": [{"index": 5466, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 16, "IndexType": 35, "Size": 4, "Name": ""}}, "nested_types": {"ElementType": {"index": 16, "spelling": "SignedCharacter", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [PDB UDT 源码行] {"ipi_index": 94087, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lcms2-sys-4.0.1/vendor/include/lcms2_plugin.h", "line": 162, "module": null, "source_string_id": 66300}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] cmsPluginTagType; TPI 0x15a2
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsPluginTagType", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [5541]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] cmsPluginTagType; TPI 0x15a5
// 原始定义文件映射: PDB_UDT_source_line
// size = 80; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 5540, "Name": "cmsPluginTagType", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 80}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5540, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5208, "FieldOffset": 0, "Name": "base"}}, "type_references": [{"index": 5208, "kind": "LF_STRUCTURE", "spelling": "_cmsPluginBaseStruct", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5540, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4736, "FieldOffset": 24, "Name": "Handler"}}, "type_references": [{"index": 4736, "kind": "LF_STRUCTURE", "spelling": "_cms_typehandler_struct", "status": "PDB_record_name"}]}
// [PDB UDT 源码行] {"ipi_index": 94135, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lcms2-sys-4.0.1/vendor/include/lcms2_plugin.h", "line": 419, "module": null, "source_string_id": 66300}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] cmsPluginTag; TPI 0x15b0
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsPluginTag", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [5555]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] cmsPluginTag; TPI 0x15b3
// 原始定义文件映射: PDB_UDT_source_line
// size = 128; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None"], "FieldList": 5554, "Name": "cmsPluginTag", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 128}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5554, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5208, "FieldOffset": 0, "Name": "base"}}, "type_references": [{"index": 5208, "kind": "LF_STRUCTURE", "spelling": "_cmsPluginBaseStruct", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5554, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4719, "FieldOffset": 24, "Name": "Signature"}}, "type_references": [{"index": 4719, "kind": "LF_ENUM", "spelling": "<unnamed-tag>", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5554, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4815, "FieldOffset": 32, "Name": "Descriptor"}}, "type_references": [{"index": 4815, "kind": "LF_STRUCTURE", "spelling": "cmsTagDescriptor", "status": "PDB_record_name"}]}
// [PDB UDT 源码行] {"ipi_index": 94142, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lcms2-sys-4.0.1/vendor/include/lcms2_plugin.h", "line": 444, "module": null, "source_string_id": 66300}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] _cmsStageToneCurvesData; TPI 0x15bb
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cmsStageToneCurvesData", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [5567]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] _cmsStageToneCurvesData; TPI 0x15bf
// 原始定义文件映射: PDB_UDT_source_line
// size = 16; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 5566, "Name": "_cmsStageToneCurvesData", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 16}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5566, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 0, "Name": "nCurves"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5566, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5565, "FieldOffset": 8, "Name": "TheCurves"}}, "type_references": [{"index": 5565, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4942, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4942, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4941, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4941, "kind": "LF_STRUCTURE", "spelling": "_cms_curve_struct", "status": "PDB_record_name"}}}}}]}
// [PDB UDT 源码行] {"ipi_index": 94182, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lcms2-sys-4.0.1/vendor/include/lcms2_plugin.h", "line": 516, "module": null, "source_string_id": 66300}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] _cmsStageMatrixData; TPI 0x15c0
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cmsStageMatrixData", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [5571]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] _cmsStageMatrixData; TPI 0x15c3
// 原始定义文件映射: PDB_UDT_source_line
// size = 16; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 5570, "Name": "_cmsStageMatrixData", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 16}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5570, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1601, "FieldOffset": 0, "Name": "Double"}}, "type_references": [{"index": 1601, "spelling": "Float64 / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5570, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1601, "FieldOffset": 8, "Name": "Offset"}}, "type_references": [{"index": 1601, "spelling": "Float64 / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 94183, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lcms2-sys-4.0.1/vendor/include/lcms2_plugin.h", "line": 523, "module": null, "source_string_id": 66300}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] _cmsStageCLutData; TPI 0x15c4
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cmsStageCLutData", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [5577]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] _cmsStageCLutData::<unnamed-tag>; TPI 0x15c7
// 原始定义文件映射: PDB_UDT_source_line
// size = 8; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_UNION", "Union": {"MemberCount": 2, "Options": ["None", "Nested", "Sealed"], "FieldList": 5574, "Name": "_cmsStageCLutData::<unnamed-tag>", "UniqueName": "", "Size": 8}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5574, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1569, "FieldOffset": 0, "Name": "T"}}, "type_references": [{"index": 1569, "spelling": "UInt16Short / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5574, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1600, "FieldOffset": 0, "Name": "TFloat"}}, "type_references": [{"index": 1600, "spelling": "Float32 / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 94184, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lcms2-sys-4.0.1/vendor/include/lcms2_plugin.h", "line": 532, "module": null, "source_string_id": 66300}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] _cmsStageCLutData; TPI 0x15c9
// 原始定义文件映射: PDB_UDT_source_line
// size = 24; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 5, "Options": ["None", "ContainsNestedClass"], "FieldList": 5576, "Name": "_cmsStageCLutData", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 24}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5576, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5575, "FieldOffset": 0, "Name": "Tab"}}, "type_references": [{"index": 5575, "kind": "LF_UNION", "spelling": "_cmsStageCLutData::<unnamed-tag>", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5576, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4945, "FieldOffset": 8, "Name": "Params"}}, "type_references": [{"index": 4945, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4944, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4944, "kind": "LF_STRUCTURE", "spelling": "_cms_interp_struc", "status": "PDB_record_name"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5576, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 16, "Name": "nEntries"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5576, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 116, "FieldOffset": 20, "Name": "HasFloatValues"}}, "type_references": [{"index": 116, "spelling": "Int32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5576, "order": 4, "kind": "LF_NESTTYPE", "record": {"Kind": "LF_NESTTYPE", "NestedType": {"Type": 5575, "Name": ""}}, "type_references": [{"index": 5575, "kind": "LF_UNION", "spelling": "_cmsStageCLutData::<unnamed-tag>", "status": "PDB_record_name"}]}
// [PDB UDT 源码行] {"ipi_index": 94185, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lcms2-sys-4.0.1/vendor/include/lcms2_plugin.h", "line": 530, "module": null, "source_string_id": 66300}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] cmsPluginParametricCurves; TPI 0x1738
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsPluginParametricCurves", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [5947]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] cmsPluginParametricCurves; TPI 0x173b
// 原始定义文件映射: PDB_UDT_source_line
// size = 200; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 5, "Options": ["None"], "FieldList": 5946, "Name": "cmsPluginParametricCurves", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 200}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5946, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5208, "FieldOffset": 0, "Name": "base"}}, "type_references": [{"index": 5208, "kind": "LF_STRUCTURE", "spelling": "_cmsPluginBaseStruct", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5946, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 24, "Name": "nFunctions"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5946, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5918, "FieldOffset": 28, "Name": "FunctionTypes"}}, "type_references": [{"index": 5918, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 117, "IndexType": 35, "Size": 80, "Name": ""}}, "nested_types": {"ElementType": {"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5946, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5918, "FieldOffset": 108, "Name": "ParameterCount"}}, "type_references": [{"index": 5918, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 117, "IndexType": 35, "Size": 80, "Name": ""}}, "nested_types": {"ElementType": {"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5946, "order": 4, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4953, "FieldOffset": 192, "Name": "Evaluator"}}, "type_references": [{"index": 4953, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4952, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4952, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 65, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 3, "ArgumentList": 4951}}, "nested_types": {}}}}]}
// [PDB UDT 源码行] {"ipi_index": 94546, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lcms2-sys-4.0.1/vendor/include/lcms2_plugin.h", "line": 332, "module": null, "source_string_id": 66300}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] cmsPluginOptimization; TPI 0x1771
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsPluginOptimization", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [6004]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] cmsPluginOptimization; TPI 0x1774
// 原始定义文件映射: PDB_UDT_source_line
// size = 32; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 6003, "Name": "cmsPluginOptimization", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 32}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6003, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5208, "FieldOffset": 0, "Name": "base"}}, "type_references": [{"index": 5208, "kind": "LF_STRUCTURE", "spelling": "_cmsPluginBaseStruct", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6003, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5997, "FieldOffset": 24, "Name": "OptimizePtr"}}, "type_references": [{"index": 5997, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 5996, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 5996, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 116, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 5, "ArgumentList": 5995}}, "nested_types": {}}}}]}
// [PDB UDT 源码行] {"ipi_index": 94588, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lcms2-sys-4.0.1/vendor/include/lcms2_plugin.h", "line": 577, "module": null, "source_string_id": 66300}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] cmsFormatter; TPI 0x179c
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_UNION", "Union": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsFormatter", "UniqueName": "", "Size": 0}}
// 前向定义候选（仅名称关联）: [6048]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] cmsFormatter; TPI 0x17a0
// 原始定义文件映射: PDB_UDT_source_line
// size = 8; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_UNION", "Union": {"MemberCount": 2, "Options": ["None", "Sealed"], "FieldList": 6047, "Name": "cmsFormatter", "UniqueName": "", "Size": 8}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6047, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5123, "FieldOffset": 0, "Name": "Fmt16"}}, "type_references": [{"index": 5123, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 5122, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 5122, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 1568, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 4, "ArgumentList": 5121}}, "nested_types": {}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6047, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5126, "FieldOffset": 0, "Name": "FmtFloat"}}, "type_references": [{"index": 5126, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 5125, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 5125, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 1568, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 4, "ArgumentList": 5124}}, "nested_types": {}}}}]}
// [PDB UDT 源码行] {"ipi_index": 94624, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lcms2-sys-4.0.1/vendor/include/lcms2_plugin.h", "line": 361, "module": null, "source_string_id": 66300}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] <unnamed-tag>; TPI 0x17aa
// 原始定义文件映射: PDB_UDT_source_line
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 2, "Options": ["None"], "FieldList": 6057, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6057, "order": 0, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "cmsFormatterInput"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6057, "order": 1, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "cmsFormatterOutput"}}, "type_references": []}
// [PDB UDT 源码行] {"ipi_index": 94633, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lcms2-sys-4.0.1/vendor/include/lcms2_plugin.h", "line": 370, "module": null, "source_string_id": 66300}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] cmsPluginFormatters; TPI 0x17b1
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsPluginFormatters", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [6068]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] cmsPluginFormatters; TPI 0x17b4
// 原始定义文件映射: PDB_UDT_source_line
// size = 32; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 6067, "Name": "cmsPluginFormatters", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 32}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6067, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5208, "FieldOffset": 0, "Name": "base"}}, "type_references": [{"index": 5208, "kind": "LF_STRUCTURE", "spelling": "_cmsPluginBaseStruct", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6067, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6061, "FieldOffset": 24, "Name": "FormattersFactory"}}, "type_references": [{"index": 6061, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 6060, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 6060, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 6044, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 3, "ArgumentList": 6059}}, "nested_types": {}}}}]}
// [PDB UDT 源码行] {"ipi_index": 94637, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lcms2-sys-4.0.1/vendor/include/lcms2_plugin.h", "line": 377, "module": null, "source_string_id": 66300}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] cmsPluginRenderingIntent; TPI 0x17f8
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsPluginRenderingIntent", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [6139]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] cmsPluginRenderingIntent; TPI 0x17fb
// 原始定义文件映射: PDB_UDT_source_line
// size = 296; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 6138, "Name": "cmsPluginRenderingIntent", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 296}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6138, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5208, "FieldOffset": 0, "Name": "base"}}, "type_references": [{"index": 5208, "kind": "LF_STRUCTURE", "spelling": "_cmsPluginBaseStruct", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6138, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 24, "Name": "Intent"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6138, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6118, "FieldOffset": 32, "Name": "Link"}}, "type_references": [{"index": 6118, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 6117, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 6117, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 5067, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 7, "ArgumentList": 6116}}, "nested_types": {}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6138, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4741, "FieldOffset": 40, "Name": "Description"}}, "type_references": [{"index": 4741, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 112, "IndexType": 35, "Size": 256, "Name": ""}}, "nested_types": {"ElementType": {"index": 112, "spelling": "NarrowCharacter", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [PDB UDT 源码行] {"ipi_index": 94797, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lcms2-sys-4.0.1/vendor/include/lcms2_plugin.h", "line": 469, "module": null, "source_string_id": 66300}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] cmsPluginInterpolation; TPI 0x1d66
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsPluginInterpolation", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [7529]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] cmsPluginInterpolation; TPI 0x1d69
// 原始定义文件映射: PDB_UDT_source_line
// size = 32; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 7528, "Name": "cmsPluginInterpolation", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 32}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7528, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5208, "FieldOffset": 0, "Name": "base"}}, "type_references": [{"index": 5208, "kind": "LF_STRUCTURE", "spelling": "_cmsPluginBaseStruct", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7528, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7523, "FieldOffset": 24, "Name": "InterpolatorsFactory"}}, "type_references": [{"index": 7523, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 7518, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 7518, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 4958, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 3, "ArgumentList": 5494}}, "nested_types": {}}}}]}
// [PDB UDT 源码行] {"ipi_index": 112642, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lcms2-sys-4.0.1/vendor/include/lcms2_plugin.h", "line": 316, "module": null, "source_string_id": 66300}
// === 完整 TPI 结构关系引用记录；原文不转换成猜测的 Rust ABI ===
// [TPI 0x1100] {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 3, "Modifiers": ["None", "Const"]}}
// [TPI 0x1101] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4352, "Attrs": 65548}}
// [TPI 0x114a] {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 33, "Modifiers": ["None", "Const"]}}
// [TPI 0x115a] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4426, "Attrs": 65548}}
// [TPI 0x1209] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 1539, "Attrs": 65548}}
// [TPI 0x126e] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1093812784, "Name": "cmsSigAToB0Tag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1093812785, "Name": "cmsSigAToB1Tag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1093812786, "Name": "cmsSigAToB2Tag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1649957210, "Name": "cmsSigBlueColorantTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1649957210, "Name": "cmsSigBlueMatrixColumnTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1649693251, "Name": "cmsSigBlueTRCTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1110589744, "Name": "cmsSigBToA0Tag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1110589745, "Name": "cmsSigBToA1Tag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1110589746, "Name": "cmsSigBToA2Tag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1667329140, "Name": "cmsSigCalibrationDateTimeTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1952543335, "Name": "cmsSigCharTargetTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1667785060, "Name": "cmsSigChromaticAdaptationTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1667789421, "Name": "cmsSigChromaticityTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1668051567, "Name": "cmsSigColorantOrderTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1668051572, "Name": "cmsSigColorantTableTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1668050804, "Name": "cmsSigColorantTableOutTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1667852659, "Name": "cmsSigColorimetricIntentImageStateTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1668313716, "Name": "cmsSigCopyrightTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1668441193, "Name": "cmsSigCrdInfoTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1684108385, "Name": "cmsSigDataTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1685350765, "Name": "cmsSigDateTimeTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1684893284, "Name": "cmsSigDeviceMfgDescTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1684890724, "Name": "cmsSigDeviceModelDescTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1684371059, "Name": "cmsSigDeviceSettingsTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1144144432, "Name": "cmsSigDToB0Tag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1144144433, "Name": "cmsSigDToB1Tag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1144144434, "Name": "cmsSigDToB2Tag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1144144435, "Name": "cmsSigDToB3Tag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1110590512, "Name": "cmsSigBToD0Tag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1110590513, "Name": "cmsSigBToD1Tag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1110590514, "Name": "cmsSigBToD2Tag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1110590515, "Name": "cmsSigBToD3Tag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1734438260, "Name": "cmsSigGamutTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1800688195, "Name": "cmsSigGrayTRCTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1733843290, "Name": "cmsSigGreenColorantTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1733843290, "Name": "cmsSigGreenMatrixColumnTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1733579331, "Name": "cmsSigGreenTRCTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1819635049, "Name": "cmsSigLuminanceTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1835360627, "Name": "cmsSigMeasurementTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1651208308, "Name": "cmsSigMediaBlackPointTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 2004119668, "Name": "cmsSigMediaWhitePointTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1852010348, "Name": "cmsSigNamedColorTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1852009522, "Name": "cmsSigNamedColor2Tag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1919251312, "Name": "cmsSigOutputResponseTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1919510320, "Name": "cmsSigPerceptualRenderingIntentGamutTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1886545200, "Name": "cmsSigPreview0Tag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1886545201, "Name": "cmsSigPreview1Tag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1886545202, "Name": "cmsSigPreview2Tag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1684370275, "Name": "cmsSigProfileDescriptionTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1685283693, "Name": "cmsSigProfileDescriptionMLTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1886610801, "Name": "cmsSigProfileSequenceDescTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1886611812, "Name": "cmsSigProfileSequenceIdTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1886610480, "Name": "cmsSigPs2CRD0Tag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1886610481, "Name": "cmsSigPs2CRD1Tag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1886610482, "Name": "cmsSigPs2CRD2Tag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1886610483, "Name": "cmsSigPs2CRD3Tag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1886597747, "Name": "cmsSigPs2CSATag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1886597737, "Name": "cmsSigPs2RenderingIntentTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1918392666, "Name": "cmsSigRedColorantTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1918392666, "Name": "cmsSigRedMatrixColumnTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1918128707, "Name": "cmsSigRedTRCTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1919510322, "Name": "cmsSigSaturationRenderingIntentGamutTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1935897188, "Name": "cmsSigScreeningDescTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1935897198, "Name": "cmsSigScreeningTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1952801640, "Name": "cmsSigTechnologyTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1650877472, "Name": "cmsSigUcrBgTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1987405156, "Name": "cmsSigViewingCondDescTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1986618743, "Name": "cmsSigViewingConditionsTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1986226036, "Name": "cmsSigVcgtTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1835365473, "Name": "cmsSigMetaTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1667851120, "Name": "cmsSigcicpTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1634890867, "Name": "cmsSigArgyllArtsTag"}}]}
// [TPI 0x126f] {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 72, "Options": ["None"], "FieldList": 4718, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [TPI 0x1272] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cms_io_handler", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1273] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4722, "Attrs": 65548}}
// [TPI 0x1274] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cmsContext_struct", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1275] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4724, "Attrs": 65548}}
// [TPI 0x1280] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cms_typehandler_struct", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1281] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4736, "Attrs": 65548}}
// [TPI 0x1285] {"Kind": "LF_ARRAY", "Array": {"ElementType": 112, "IndexType": 35, "Size": 256, "Name": ""}}
// [TPI 0x1286] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [4723, 1539, 117, 117]}}
// [TPI 0x1287] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 117, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 4, "ArgumentList": 4742}}
// [TPI 0x1288] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4743, "Attrs": 65548}}
// [TPI 0x1289] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [4723, 117]}}
// [TPI 0x128a] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 116, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 2, "ArgumentList": 4745}}
// [TPI 0x128b] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4746, "Attrs": 65548}}
// [TPI 0x128c] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [4723]}}
// [TPI 0x128d] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 116, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 1, "ArgumentList": 4748}}
// [TPI 0x128e] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4749, "Attrs": 65548}}
// [TPI 0x128f] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 117, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 1, "ArgumentList": 4748}}
// [TPI 0x1290] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4751, "Attrs": 65548}}
// [TPI 0x1291] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [4723, 117, 4353]}}
// [TPI 0x1292] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 116, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 3, "ArgumentList": 4753}}
// [TPI 0x1293] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4754, "Attrs": 65548}}
// [TPI 0x1294] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 0, "Name": "stream"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4725, "FieldOffset": 8, "Name": "ContextID"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 16, "Name": "UsedSpace"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 20, "Name": "ReportedSize"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4741, "FieldOffset": 24, "Name": "PhysicalFile"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4744, "FieldOffset": 280, "Name": "Read"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4747, "FieldOffset": 288, "Name": "Seek"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4750, "FieldOffset": 296, "Name": "Close"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4752, "FieldOffset": 304, "Name": "Tell"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4755, "FieldOffset": 312, "Name": "Write"}}]}
// [TPI 0x1295] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 10, "Options": ["None"], "FieldList": 4756, "Name": "_cms_io_handler", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 320}}
// [TPI 0x12a3] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1667789421, "Name": "cmsSigChromaticityType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1667851120, "Name": "cmsSigcicpType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1668051567, "Name": "cmsSigColorantOrderType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1668051572, "Name": "cmsSigColorantTableType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1668441193, "Name": "cmsSigCrdInfoType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1668641398, "Name": "cmsSigCurveType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1684108385, "Name": "cmsSigDataType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1684628340, "Name": "cmsSigDictType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1685350765, "Name": "cmsSigDateTimeType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1684371059, "Name": "cmsSigDeviceSettingsType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1835430962, "Name": "cmsSigLut16Type"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1835430961, "Name": "cmsSigLut8Type"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1832993312, "Name": "cmsSigLutAtoBType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1833058592, "Name": "cmsSigLutBtoAType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1835360627, "Name": "cmsSigMeasurementType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1835824483, "Name": "cmsSigMultiLocalizedUnicodeType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1836082548, "Name": "cmsSigMultiProcessElementType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1852010348, "Name": "cmsSigNamedColorType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1852009522, "Name": "cmsSigNamedColor2Type"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1885434465, "Name": "cmsSigParametricCurveType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1886610801, "Name": "cmsSigProfileSequenceDescType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1886611812, "Name": "cmsSigProfileSequenceIdType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1919120178, "Name": "cmsSigResponseCurveSet16Type"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1936077618, "Name": "cmsSigS15Fixed16ArrayType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1935897198, "Name": "cmsSigScreeningType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1936287520, "Name": "cmsSigSignatureType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1952807028, "Name": "cmsSigTextType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1684370275, "Name": "cmsSigTextDescriptionType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1969632050, "Name": "cmsSigU16Fixed16ArrayType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1650877472, "Name": "cmsSigUcrBgType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1969828150, "Name": "cmsSigUInt16ArrayType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1969828658, "Name": "cmsSigUInt32ArrayType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1969829428, "Name": "cmsSigUInt64ArrayType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1969827896, "Name": "cmsSigUInt8ArrayType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1986226036, "Name": "cmsSigVcgtType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1986618743, "Name": "cmsSigViewingConditionsType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1482250784, "Name": "cmsSigXYZType"}}]}
// [TPI 0x12a4] {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 37, "Options": ["None"], "FieldList": 4771, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [TPI 0x12a5] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [4737, 4723, 1653, 117]}}
// [TPI 0x12a6] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 1539, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 4, "ArgumentList": 4773}}
// [TPI 0x12a7] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4774, "Attrs": 65548}}
// [TPI 0x12a8] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [4737, 4723, 1539, 117]}}
// [TPI 0x12a9] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 116, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 4, "ArgumentList": 4776}}
// [TPI 0x12aa] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4777, "Attrs": 65548}}
// [TPI 0x12ab] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [4737, 4353, 117]}}
// [TPI 0x12ac] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 1539, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 3, "ArgumentList": 4779}}
// [TPI 0x12ad] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4780, "Attrs": 65548}}
// [TPI 0x12ae] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [4737, 1539]}}
// [TPI 0x12af] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 3, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 2, "ArgumentList": 4782}}
// [TPI 0x12b0] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4783, "Attrs": 65548}}
// [TPI 0x12b1] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4772, "FieldOffset": 0, "Name": "Signature"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4775, "FieldOffset": 8, "Name": "ReadPtr"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4778, "FieldOffset": 16, "Name": "WritePtr"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4781, "FieldOffset": 24, "Name": "DupPtr"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4784, "FieldOffset": 32, "Name": "FreePtr"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4725, "FieldOffset": 40, "Name": "ContextID"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 48, "Name": "ICCVersion"}}]}
// [TPI 0x12b2] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 7, "Options": ["None"], "FieldList": 4785, "Name": "_cms_typehandler_struct", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 56}}
// [TPI 0x12b7] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [4725, 117]}}
// [TPI 0x12b8] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 1539, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 2, "ArgumentList": 4791}}
// [TPI 0x12b9] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4792, "Attrs": 65548}}
// [TPI 0x12ba] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [4725, 1539]}}
// [TPI 0x12bb] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 3, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 2, "ArgumentList": 4794}}
// [TPI 0x12bc] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4795, "Attrs": 65548}}
// [TPI 0x12bd] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [4725, 1539, 117]}}
// [TPI 0x12be] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 1539, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 3, "ArgumentList": 4797}}
// [TPI 0x12bf] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4798, "Attrs": 65548}}
// [TPI 0x12c0] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [4725, 117, 117]}}
// [TPI 0x12c1] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 1539, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 3, "ArgumentList": 4800}}
// [TPI 0x12c2] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4801, "Attrs": 65548}}
// [TPI 0x12c3] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [4725, 4353, 117]}}
// [TPI 0x12c4] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 1539, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 3, "ArgumentList": 4803}}
// [TPI 0x12c5] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4804, "Attrs": 65548}}
// [TPI 0x12cf] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsTagDescriptor", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x12d4] {"Kind": "LF_ARRAY", "Array": {"ElementType": 4772, "IndexType": 35, "Size": 80, "Name": ""}}
// [TPI 0x12d5] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [65, 4353]}}
// [TPI 0x12d6] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 4772, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 2, "ArgumentList": 4821}}
// [TPI 0x12d7] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4822, "Attrs": 65548}}
// [TPI 0x12d8] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 0, "Name": "ElemCount"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 4, "Name": "nSupportedTypes"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4820, "FieldOffset": 8, "Name": "SupportedTypes"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4823, "FieldOffset": 88, "Name": "DecideType"}}]}
// [TPI 0x12d9] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 4824, "Name": "cmsTagDescriptor", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 96}}
// [TPI 0x12dc] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [4725]}}
// [TPI 0x12dd] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 1539, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 1, "ArgumentList": 4828}}
// [TPI 0x134d] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cms_curve_struct", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x134e] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4941, "Attrs": 65548}}
// [TPI 0x1350] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cms_interp_struc", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1351] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4944, "Attrs": 65548}}
// [TPI 0x1355] {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 65, "Modifiers": ["None", "Const"]}}
// [TPI 0x1356] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4949, "Attrs": 65548}}
// [TPI 0x1357] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [116, 4950, 65]}}
// [TPI 0x1358] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 65, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 3, "ArgumentList": 4951}}
// [TPI 0x1359] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4952, "Attrs": 65548}}
// [TPI 0x135d] {"Kind": "LF_ARRAY", "Array": {"ElementType": 117, "IndexType": 35, "Size": 60, "Name": ""}}
// [TPI 0x135e] {"Kind": "LF_UNION", "Union": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsInterpFunction", "UniqueName": "", "Size": 0}}
// [TPI 0x135f] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4725, "FieldOffset": 0, "Name": "ContextID"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 8, "Name": "dwFlags"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 12, "Name": "nInputs"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 16, "Name": "nOutputs"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4957, "FieldOffset": 20, "Name": "nSamples"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4957, "FieldOffset": 80, "Name": "Domain"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4957, "FieldOffset": 140, "Name": "opta"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4353, "FieldOffset": 200, "Name": "Table"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4958, "FieldOffset": 208, "Name": "Interpolation"}}]}
// [TPI 0x1360] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 9, "Options": ["None"], "FieldList": 4959, "Name": "_cms_interp_struc", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 216}}
// [TPI 0x1364] {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 4944, "Modifiers": ["None", "Const"]}}
// [TPI 0x1365] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4964, "Attrs": 65548}}
// [TPI 0x1366] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [4442, 1569, 4965]}}
// [TPI 0x1367] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 3, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 3, "ArgumentList": 4966}}
// [TPI 0x1368] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4967, "Attrs": 65548}}
// [TPI 0x1369] {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 64, "Modifiers": ["None", "Const"]}}
// [TPI 0x136a] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4969, "Attrs": 65548}}
// [TPI 0x136b] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [4970, 1600, 4965]}}
// [TPI 0x136c] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 3, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 3, "ArgumentList": 4971}}
// [TPI 0x136d] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4972, "Attrs": 65548}}
// [TPI 0x136e] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4968, "FieldOffset": 0, "Name": "Lerp16"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4973, "FieldOffset": 0, "Name": "LerpFloat"}}]}
// [TPI 0x136f] {"Kind": "LF_UNION", "Union": {"MemberCount": 2, "Options": ["None", "Sealed"], "FieldList": 4974, "Name": "cmsInterpFunction", "UniqueName": "", "Size": 8}}
// [TPI 0x1376] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cmsPipeline_struct", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x13a9] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsMAT3", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x13aa] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsVEC3", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x13ab] {"Kind": "LF_ARRAY", "Array": {"ElementType": 5034, "IndexType": 35, "Size": 72, "Name": ""}}
// [TPI 0x13ac] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5035, "FieldOffset": 0, "Name": "v"}}]}
// [TPI 0x13ad] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 1, "Options": ["None"], "FieldList": 5036, "Name": "cmsMAT3", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 72}}
// [TPI 0x13ae] {"Kind": "LF_ARRAY", "Array": {"ElementType": 65, "IndexType": 35, "Size": 24, "Name": ""}}
// [TPI 0x13af] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5038, "FieldOffset": 0, "Name": "n"}}]}
// [TPI 0x13b0] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 1, "Options": ["None"], "FieldList": 5039, "Name": "cmsVEC3", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 24}}
// [TPI 0x13cb] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4982, "Attrs": 65548}}
// [TPI 0x13f9] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cmstransform_struct", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x13fa] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 5113, "Attrs": 65548}}
// [TPI 0x13fb] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsStride", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x13fc] {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 5115, "Modifiers": ["None", "Const"]}}
// [TPI 0x13fd] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 5116, "Attrs": 65548}}
// [TPI 0x13fe] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [5114, 4353, 1539, 117, 117, 5117]}}
// [TPI 0x13ff] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 3, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 6, "ArgumentList": 5118}}
// [TPI 0x1400] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 5119, "Attrs": 65548}}
// [TPI 0x1401] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [5114, 1569, 1568, 117]}}
// [TPI 0x1402] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 1568, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 4, "ArgumentList": 5121}}
// [TPI 0x1403] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 5122, "Attrs": 65548}}
// [TPI 0x1404] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [5114, 1600, 1568, 117]}}
// [TPI 0x1405] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 1568, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 4, "ArgumentList": 5124}}
// [TPI 0x1406] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 5125, "Attrs": 65548}}
// [TPI 0x140a] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [5114, 4353, 1539, 117, 117]}}
// [TPI 0x140b] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 3, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 5, "ArgumentList": 5130}}
// [TPI 0x140c] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 5131, "Attrs": 65548}}
// [TPI 0x140f] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 0, "Name": "BytesPerLineIn"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 4, "Name": "BytesPerLineOut"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 8, "Name": "BytesPerPlaneIn"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 12, "Name": "BytesPerPlaneOut"}}]}
// [TPI 0x1410] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 5135, "Name": "cmsStride", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 16}}
// [TPI 0x144f] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 5120, "Attrs": 65548}}
// [TPI 0x1450] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4796, "Attrs": 65548}}
// [TPI 0x1451] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 5067, "Attrs": 65548}}
// [TPI 0x1452] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [5199, 4617, 5200, 5201, 1653, 1653, 1653]}}
// [TPI 0x1453] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 116, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 7, "ArgumentList": 5202}}
// [TPI 0x1454] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 5203, "Attrs": 65548}}
// [TPI 0x1458] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cmsPluginBaseStruct", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1459] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 5208, "Attrs": 65548}}
// [TPI 0x145c] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 0, "Name": "Magic"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 4, "Name": "ExpectedVersion"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 8, "Name": "Type"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5209, "FieldOffset": 16, "Name": "Next"}}]}
// [TPI 0x145d] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 5212, "Name": "_cmsPluginBaseStruct", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 24}}
// [TPI 0x145e] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsPluginTransform", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1460] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 5132, "Attrs": 65548}}
// [TPI 0x1461] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [5216, 4617, 5200, 5201, 1653, 1653, 1653]}}
// [TPI 0x1462] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 116, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 7, "ArgumentList": 5217}}
// [TPI 0x1463] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 5218, "Attrs": 65548}}
// [TPI 0x1464] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5219, "FieldOffset": 0, "Name": "legacy_xform"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5204, "FieldOffset": 0, "Name": "xform"}}]}
// [TPI 0x1465] {"Kind": "LF_UNION", "Union": {"MemberCount": 2, "Options": ["None", "Nested", "Sealed"], "FieldList": 5220, "Name": "cmsPluginTransform::<unnamed-tag>", "UniqueName": "", "Size": 8}}
// [TPI 0x1466] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5208, "FieldOffset": 0, "Name": "base"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5221, "FieldOffset": 24, "Name": "factories"}}, {"Kind": "LF_NESTTYPE", "NestedType": {"Type": 5221, "Name": ""}}]}
// [TPI 0x1467] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "ContainsNestedClass"], "FieldList": 5222, "Name": "cmsPluginTransform", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 32}}
// [TPI 0x14a6] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsPluginMemHandler", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x14ab] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5208, "FieldOffset": 0, "Name": "base"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4793, "FieldOffset": 24, "Name": "MallocPtr"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4796, "FieldOffset": 32, "Name": "FreePtr"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4799, "FieldOffset": 40, "Name": "ReallocPtr"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4793, "FieldOffset": 48, "Name": "MallocZeroPtr"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4802, "FieldOffset": 56, "Name": "CallocPtr"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4805, "FieldOffset": 64, "Name": "DupPtr"}}]}
// [TPI 0x14ac] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 7, "Options": ["None"], "FieldList": 5291, "Name": "cmsPluginMemHandler", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 72}}
// [TPI 0x14e2] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 116, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 2, "ArgumentList": 4794}}
// [TPI 0x14e4] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4829, "Attrs": 65548}}
// [TPI 0x14e5] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 5346, "Attrs": 65548}}
// [TPI 0x14e8] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsPluginMutex", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x14ea] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5208, "FieldOffset": 0, "Name": "base"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5348, "FieldOffset": 24, "Name": "CreateMutexPtr"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4796, "FieldOffset": 32, "Name": "DestroyMutexPtr"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5349, "FieldOffset": 40, "Name": "LockMutexPtr"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4796, "FieldOffset": 48, "Name": "UnlockMutexPtr"}}]}
// [TPI 0x14eb] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 5, "Options": ["None"], "FieldList": 5354, "Name": "cmsPluginMutex", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 56}}
// [TPI 0x14ed] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsPluginParalellization", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x14ef] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5208, "FieldOffset": 0, "Name": "base"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 116, "FieldOffset": 24, "Name": "MaxWorkers"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 28, "Name": "WorkerFlags"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5120, "FieldOffset": 32, "Name": "SchedulerFn"}}]}
// [TPI 0x14f0] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 5359, "Name": "cmsPluginParalellization", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 40}}
// [TPI 0x1559] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cmsTagBase", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x155a] {"Kind": "LF_ARRAY", "Array": {"ElementType": 16, "IndexType": 35, "Size": 4, "Name": ""}}
// [TPI 0x155b] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4772, "FieldOffset": 0, "Name": "sig"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5466, "FieldOffset": 4, "Name": "reserved"}}]}
// [TPI 0x155c] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 5467, "Name": "_cmsTagBase", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 8}}
// [TPI 0x1576] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [117, 117, 117]}}
// [TPI 0x15a2] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsPluginTagType", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x15a4] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5208, "FieldOffset": 0, "Name": "base"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4736, "FieldOffset": 24, "Name": "Handler"}}]}
// [TPI 0x15a5] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 5540, "Name": "cmsPluginTagType", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 80}}
// [TPI 0x15b0] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsPluginTag", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x15b2] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5208, "FieldOffset": 0, "Name": "base"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4719, "FieldOffset": 24, "Name": "Signature"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4815, "FieldOffset": 32, "Name": "Descriptor"}}]}
// [TPI 0x15b3] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None"], "FieldList": 5554, "Name": "cmsPluginTag", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 128}}
// [TPI 0x15bb] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cmsStageToneCurvesData", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x15bd] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4942, "Attrs": 65548}}
// [TPI 0x15be] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 0, "Name": "nCurves"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5565, "FieldOffset": 8, "Name": "TheCurves"}}]}
// [TPI 0x15bf] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 5566, "Name": "_cmsStageToneCurvesData", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 16}}
// [TPI 0x15c0] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cmsStageMatrixData", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x15c2] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1601, "FieldOffset": 0, "Name": "Double"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1601, "FieldOffset": 8, "Name": "Offset"}}]}
// [TPI 0x15c3] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 5570, "Name": "_cmsStageMatrixData", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 16}}
// [TPI 0x15c4] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cmsStageCLutData", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x15c6] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1569, "FieldOffset": 0, "Name": "T"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1600, "FieldOffset": 0, "Name": "TFloat"}}]}
// [TPI 0x15c7] {"Kind": "LF_UNION", "Union": {"MemberCount": 2, "Options": ["None", "Nested", "Sealed"], "FieldList": 5574, "Name": "_cmsStageCLutData::<unnamed-tag>", "UniqueName": "", "Size": 8}}
// [TPI 0x15c8] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5575, "FieldOffset": 0, "Name": "Tab"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4945, "FieldOffset": 8, "Name": "Params"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 16, "Name": "nEntries"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 116, "FieldOffset": 20, "Name": "HasFloatValues"}}, {"Kind": "LF_NESTTYPE", "NestedType": {"Type": 5575, "Name": ""}}]}
// [TPI 0x15c9] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 5, "Options": ["None", "ContainsNestedClass"], "FieldList": 5576, "Name": "_cmsStageCLutData", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 24}}
// [TPI 0x171e] {"Kind": "LF_ARRAY", "Array": {"ElementType": 117, "IndexType": 35, "Size": 80, "Name": ""}}
// [TPI 0x1738] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsPluginParametricCurves", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x173a] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5208, "FieldOffset": 0, "Name": "base"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 24, "Name": "nFunctions"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5918, "FieldOffset": 28, "Name": "FunctionTypes"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5918, "FieldOffset": 108, "Name": "ParameterCount"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4953, "FieldOffset": 192, "Name": "Evaluator"}}]}
// [TPI 0x173b] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 5, "Options": ["None"], "FieldList": 5946, "Name": "cmsPluginParametricCurves", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 200}}
// [TPI 0x176b] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [5201, 117, 1653, 1653, 1653]}}
// [TPI 0x176c] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 116, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 5, "ArgumentList": 5995}}
// [TPI 0x176d] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 5996, "Attrs": 65548}}
// [TPI 0x1771] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsPluginOptimization", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1773] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5208, "FieldOffset": 0, "Name": "base"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5997, "FieldOffset": 24, "Name": "OptimizePtr"}}]}
// [TPI 0x1774] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 6003, "Name": "cmsPluginOptimization", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 32}}
// [TPI 0x179c] {"Kind": "LF_UNION", "Union": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsFormatter", "UniqueName": "", "Size": 0}}
// [TPI 0x179f] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5123, "FieldOffset": 0, "Name": "Fmt16"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5126, "FieldOffset": 0, "Name": "FmtFloat"}}]}
// [TPI 0x17a0] {"Kind": "LF_UNION", "Union": {"MemberCount": 2, "Options": ["None", "Sealed"], "FieldList": 6047, "Name": "cmsFormatter", "UniqueName": "", "Size": 8}}
// [TPI 0x17a9] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "cmsFormatterInput"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "cmsFormatterOutput"}}]}
// [TPI 0x17aa] {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 2, "Options": ["None"], "FieldList": 6057, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [TPI 0x17ab] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [117, 6058, 117]}}
// [TPI 0x17ac] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 6044, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 3, "ArgumentList": 6059}}
// [TPI 0x17ad] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 6060, "Attrs": 65548}}
// [TPI 0x17b1] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsPluginFormatters", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x17b3] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5208, "FieldOffset": 0, "Name": "base"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6061, "FieldOffset": 24, "Name": "FormattersFactory"}}]}
// [TPI 0x17b4] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 6067, "Name": "cmsPluginFormatters", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 32}}
// [TPI 0x17e4] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [4725, 117, 1653, 4617, 1652, 1601, 117]}}
// [TPI 0x17e5] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 5067, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 7, "ArgumentList": 6116}}
// [TPI 0x17e6] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 6117, "Attrs": 65548}}
// [TPI 0x17f8] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsPluginRenderingIntent", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x17fa] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5208, "FieldOffset": 0, "Name": "base"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 24, "Name": "Intent"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6118, "FieldOffset": 32, "Name": "Link"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4741, "FieldOffset": 40, "Name": "Description"}}]}
// [TPI 0x17fb] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 6138, "Name": "cmsPluginRenderingIntent", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 296}}
// [TPI 0x1d5e] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 4958, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 3, "ArgumentList": 5494}}
// [TPI 0x1d63] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 7518, "Attrs": 65548}}
// [TPI 0x1d66] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsPluginInterpolation", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1d68] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5208, "FieldOffset": 0, "Name": "base"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7523, "FieldOffset": 24, "Name": "InterpolatorsFactory"}}]}
// [TPI 0x1d69] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 7528, "Name": "cmsPluginInterpolation", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 32}}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: lcms2_plugin.h.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===
