// Generated from PDB metadata by skeleton/_tools/fill_type_structures.py.
// Version: B
// 类型元数据；原作者源码文本/定义文件见逐项映射；不生成依赖实现。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: lcms2_plugin.h.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB forward_declaration_only] cmsStride; TPI 0x103d
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsStride", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [4183]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] cmsStride; TPI 0x1057
// 原始定义文件映射: PDB_UDT_source_line
// size = 16; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 4182, "Name": "cmsStride", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 16}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4182, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 0, "Name": "BytesPerLineIn"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4182, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 4, "Name": "BytesPerLineOut"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4182, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 8, "Name": "BytesPerPlaneIn"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4182, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 12, "Name": "BytesPerPlaneOut"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 74843, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lcms2-sys-4.0.1/vendor/include/lcms2_plugin.h", "line": 588, "module": null, "source_string_id": 74842}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] _cmsPluginBaseStruct; TPI 0x10c6
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cmsPluginBaseStruct", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [4299]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] _cmsPluginBaseStruct; TPI 0x10cb
// 原始定义文件映射: PDB_UDT_source_line
// size = 24; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 4298, "Name": "_cmsPluginBaseStruct", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 24}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4298, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 0, "Name": "Magic"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4298, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 4, "Name": "ExpectedVersion"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4298, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 8, "Name": "Type"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4298, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4295, "FieldOffset": 16, "Name": "Next"}}, "type_references": [{"index": 4295, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4294, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4294, "kind": "LF_STRUCTURE", "spelling": "_cmsPluginBaseStruct", "status": "PDB_record_name"}}}]}
// [PDB UDT 源码行] {"ipi_index": 74877, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lcms2-sys-4.0.1/vendor/include/lcms2_plugin.h", "line": 214, "module": null, "source_string_id": 74842}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] cmsPluginTransform; TPI 0x10cc
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsPluginTransform", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [4309]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] cmsPluginTransform::<unnamed-tag>; TPI 0x10d3
// 原始定义文件映射: PDB_UDT_source_line
// size = 8; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_UNION", "Union": {"MemberCount": 2, "Options": ["None", "Nested", "Sealed"], "FieldList": 4306, "Name": "cmsPluginTransform::<unnamed-tag>", "UniqueName": "", "Size": 8}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4306, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4305, "FieldOffset": 0, "Name": "legacy_xform"}}, "type_references": [{"index": 4305, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4304, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4304, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 116, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 7, "ArgumentList": 4303}}, "nested_types": {}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4306, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4290, "FieldOffset": 0, "Name": "xform"}}, "type_references": [{"index": 4290, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4289, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4289, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 116, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 7, "ArgumentList": 4288}}, "nested_types": {}}}}]}
// [PDB UDT 源码行] {"ipi_index": 74879, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lcms2-sys-4.0.1/vendor/include/lcms2_plugin.h", "line": 643, "module": null, "source_string_id": 74842}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] cmsPluginTransform; TPI 0x10d5
// 原始定义文件映射: PDB_UDT_source_line
// size = 32; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "ContainsNestedClass"], "FieldList": 4308, "Name": "cmsPluginTransform", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 32}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4308, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4294, "FieldOffset": 0, "Name": "base"}}, "type_references": [{"index": 4294, "kind": "LF_STRUCTURE", "spelling": "_cmsPluginBaseStruct", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4308, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4307, "FieldOffset": 24, "Name": "factories"}}, "type_references": [{"index": 4307, "kind": "LF_UNION", "spelling": "cmsPluginTransform::<unnamed-tag>", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4308, "order": 2, "kind": "LF_NESTTYPE", "record": {"Kind": "LF_NESTTYPE", "NestedType": {"Type": 4307, "Name": ""}}, "type_references": [{"index": 4307, "kind": "LF_UNION", "spelling": "cmsPluginTransform::<unnamed-tag>", "status": "PDB_record_name"}]}
// [PDB UDT 源码行] {"ipi_index": 74880, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lcms2-sys-4.0.1/vendor/include/lcms2_plugin.h", "line": 639, "module": null, "source_string_id": 74842}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] _cms_io_handler; TPI 0x110a
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cms_io_handler", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [4389]
// ------------------------------------------------------------------
// [PDB forward_declaration_only] _cms_typehandler_struct; TPI 0x1111
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cms_typehandler_struct", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [4407]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] _cms_io_handler; TPI 0x1125
// 原始定义文件映射: PDB_UDT_source_line
// size = 320; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 10, "Options": ["None"], "FieldList": 4388, "Name": "_cms_io_handler", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 320}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4388, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 0, "Name": "stream"}}, "type_references": [{"index": 1539, "spelling": "Void / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4388, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4099, "FieldOffset": 8, "Name": "ContextID"}}, "type_references": [{"index": 4099, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4098, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4098, "kind": "LF_STRUCTURE", "spelling": "_cmsContext_struct", "status": "PDB_record_name"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4388, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 16, "Name": "UsedSpace"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4388, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 20, "Name": "ReportedSize"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4388, "order": 4, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4227, "FieldOffset": 24, "Name": "PhysicalFile"}}, "type_references": [{"index": 4227, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 112, "IndexType": 35, "Size": 256, "Name": ""}}, "nested_types": {"ElementType": {"index": 112, "spelling": "NarrowCharacter", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4388, "order": 5, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4376, "FieldOffset": 280, "Name": "Read"}}, "type_references": [{"index": 4376, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4375, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4375, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 117, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 4, "ArgumentList": 4374}}, "nested_types": {}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4388, "order": 6, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4379, "FieldOffset": 288, "Name": "Seek"}}, "type_references": [{"index": 4379, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4378, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4378, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 116, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 2, "ArgumentList": 4377}}, "nested_types": {}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4388, "order": 7, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4382, "FieldOffset": 296, "Name": "Close"}}, "type_references": [{"index": 4382, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4381, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4381, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 116, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 1, "ArgumentList": 4380}}, "nested_types": {}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4388, "order": 8, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4384, "FieldOffset": 304, "Name": "Tell"}}, "type_references": [{"index": 4384, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4383, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4383, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 117, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 1, "ArgumentList": 4380}}, "nested_types": {}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4388, "order": 9, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4387, "FieldOffset": 312, "Name": "Write"}}, "type_references": [{"index": 4387, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4386, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4386, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 116, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 3, "ArgumentList": 4385}}, "nested_types": {}}}}]}
// [PDB UDT 源码行] {"ipi_index": 74989, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lcms2-sys-4.0.1/vendor/include/lcms2_plugin.h", "line": 118, "module": null, "source_string_id": 74842}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] _cms_typehandler_struct; TPI 0x1137
// 原始定义文件映射: PDB_UDT_source_line
// size = 56; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 7, "Options": ["None"], "FieldList": 4406, "Name": "_cms_typehandler_struct", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 56}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4406, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4393, "FieldOffset": 0, "Name": "Signature"}}, "type_references": [{"index": 4393, "kind": "LF_ENUM", "spelling": "<unnamed-tag>", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4406, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4396, "FieldOffset": 8, "Name": "ReadPtr"}}, "type_references": [{"index": 4396, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4395, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4395, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 1539, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 4, "ArgumentList": 4394}}, "nested_types": {}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4406, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4399, "FieldOffset": 16, "Name": "WritePtr"}}, "type_references": [{"index": 4399, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4398, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4398, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 116, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 4, "ArgumentList": 4397}}, "nested_types": {}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4406, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4402, "FieldOffset": 24, "Name": "DupPtr"}}, "type_references": [{"index": 4402, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4401, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4401, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 1539, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 3, "ArgumentList": 4400}}, "nested_types": {}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4406, "order": 4, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4405, "FieldOffset": 32, "Name": "FreePtr"}}, "type_references": [{"index": 4405, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4404, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4404, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 3, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 2, "ArgumentList": 4403}}, "nested_types": {}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4406, "order": 5, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4099, "FieldOffset": 40, "Name": "ContextID"}}, "type_references": [{"index": 4099, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4098, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4098, "kind": "LF_STRUCTURE", "spelling": "_cmsContext_struct", "status": "PDB_record_name"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4406, "order": 6, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 48, "Name": "ICCVersion"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 74993, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lcms2-sys-4.0.1/vendor/include/lcms2_plugin.h", "line": 387, "module": null, "source_string_id": 74842}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] cmsTagDescriptor; TPI 0x113c
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsTagDescriptor", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [4422]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] cmsTagDescriptor; TPI 0x1146
// 原始定义文件映射: PDB_UDT_source_line
// size = 96; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 4421, "Name": "cmsTagDescriptor", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 96}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4421, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 0, "Name": "ElemCount"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4421, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 4, "Name": "nSupportedTypes"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4421, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4417, "FieldOffset": 8, "Name": "SupportedTypes"}}, "type_references": [{"index": 4417, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 4393, "IndexType": 35, "Size": 80, "Name": ""}}, "nested_types": {"ElementType": {"index": 4393, "kind": "LF_ENUM", "spelling": "<unnamed-tag>", "status": "PDB_record_name"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4421, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4420, "FieldOffset": 88, "Name": "DecideType"}}, "type_references": [{"index": 4420, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4419, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4419, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 4393, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 2, "ArgumentList": 4418}}, "nested_types": {}}}}]}
// [PDB UDT 源码行] {"ipi_index": 74998, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lcms2-sys-4.0.1/vendor/include/lcms2_plugin.h", "line": 430, "module": null, "source_string_id": 74842}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] _cms_interp_struc; TPI 0x11be
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cms_interp_struc", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [4558]
// ------------------------------------------------------------------
// [PDB forward_declaration_only] cmsInterpFunction; TPI 0x11cc
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_UNION", "Union": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsInterpFunction", "UniqueName": "", "Size": 0}}
// 前向定义候选（仅名称关联）: [4571]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] _cms_interp_struc; TPI 0x11ce
// 原始定义文件映射: PDB_UDT_source_line
// size = 216; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 9, "Options": ["None"], "FieldList": 4557, "Name": "_cms_interp_struc", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 216}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4557, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4099, "FieldOffset": 0, "Name": "ContextID"}}, "type_references": [{"index": 4099, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4098, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4098, "kind": "LF_STRUCTURE", "spelling": "_cmsContext_struct", "status": "PDB_record_name"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4557, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 8, "Name": "dwFlags"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4557, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 12, "Name": "nInputs"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4557, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 16, "Name": "nOutputs"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4557, "order": 4, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4555, "FieldOffset": 20, "Name": "nSamples"}}, "type_references": [{"index": 4555, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 117, "IndexType": 35, "Size": 60, "Name": ""}}, "nested_types": {"ElementType": {"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4557, "order": 5, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4555, "FieldOffset": 80, "Name": "Domain"}}, "type_references": [{"index": 4555, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 117, "IndexType": 35, "Size": 60, "Name": ""}}, "nested_types": {"ElementType": {"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4557, "order": 6, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4555, "FieldOffset": 140, "Name": "opta"}}, "type_references": [{"index": 4555, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 117, "IndexType": 35, "Size": 60, "Name": ""}}, "nested_types": {"ElementType": {"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4557, "order": 7, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4125, "FieldOffset": 200, "Name": "Table"}}, "type_references": [{"index": 4125, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4124, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4124, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 3, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 3, "spelling": "Void", "status": "CodeView_simple_type; not Rust source spelling"}}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4557, "order": 8, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4556, "FieldOffset": 208, "Name": "Interpolation"}}, "type_references": [{"index": 4556, "kind": "LF_UNION", "spelling": "cmsInterpFunction", "status": "PDB_record_name"}]}
// [PDB UDT 源码行] {"ipi_index": 76900, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lcms2-sys-4.0.1/vendor/include/lcms2_plugin.h", "line": 292, "module": null, "source_string_id": 74842}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] cmsInterpFunction; TPI 0x11db
// 原始定义文件映射: PDB_UDT_source_line
// size = 8; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_UNION", "Union": {"MemberCount": 2, "Options": ["None", "Sealed"], "FieldList": 4570, "Name": "cmsInterpFunction", "UniqueName": "", "Size": 8}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4570, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4566, "FieldOffset": 0, "Name": "Lerp16"}}, "type_references": [{"index": 4566, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4565, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4565, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 3, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 3, "ArgumentList": 4564}}, "nested_types": {}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4570, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4569, "FieldOffset": 0, "Name": "LerpFloat"}}, "type_references": [{"index": 4569, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4568, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4568, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 3, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 3, "ArgumentList": 4567}}, "nested_types": {}}}}]}
// [PDB UDT 源码行] {"ipi_index": 76902, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lcms2-sys-4.0.1/vendor/include/lcms2_plugin.h", "line": 279, "module": null, "source_string_id": 74842}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] cmsMAT3; TPI 0x11f9
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsMAT3", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [4605]
// ------------------------------------------------------------------
// [PDB forward_declaration_only] cmsVEC3; TPI 0x11fa
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsVEC3", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [4608]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] cmsMAT3; TPI 0x11fd
// 原始定义文件映射: PDB_UDT_source_line
// size = 72; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 1, "Options": ["None"], "FieldList": 4604, "Name": "cmsMAT3", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 72}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4604, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4603, "FieldOffset": 0, "Name": "v"}}, "type_references": [{"index": 4603, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 4602, "IndexType": 35, "Size": 72, "Name": ""}}, "nested_types": {"ElementType": {"index": 4602, "kind": "LF_STRUCTURE", "spelling": "cmsVEC3", "status": "PDB_record_name"}}}]}
// [PDB UDT 源码行] {"ipi_index": 76913, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lcms2-sys-4.0.1/vendor/include/lcms2_plugin.h", "line": 77, "module": null, "source_string_id": 74842}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] cmsVEC3; TPI 0x1200
// 原始定义文件映射: PDB_UDT_source_line
// size = 24; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 1, "Options": ["None"], "FieldList": 4607, "Name": "cmsVEC3", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 24}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4607, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4606, "FieldOffset": 0, "Name": "n"}}, "type_references": [{"index": 4606, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 65, "IndexType": 35, "Size": 24, "Name": ""}}, "nested_types": {"ElementType": {"index": 65, "spelling": "Float64", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [PDB UDT 源码行] {"ipi_index": 76914, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lcms2-sys-4.0.1/vendor/include/lcms2_plugin.h", "line": 71, "module": null, "source_string_id": 74842}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] cmsPluginMemHandler; TPI 0x1239
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsPluginMemHandler", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [4671]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] cmsPluginMemHandler; TPI 0x123f
// 原始定义文件映射: PDB_UDT_source_line
// size = 72; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 7, "Options": ["None"], "FieldList": 4670, "Name": "cmsPluginMemHandler", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 72}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4670, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4294, "FieldOffset": 0, "Name": "base"}}, "type_references": [{"index": 4294, "kind": "LF_STRUCTURE", "spelling": "_cmsPluginBaseStruct", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4670, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4114, "FieldOffset": 24, "Name": "MallocPtr"}}, "type_references": [{"index": 4114, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4113, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4113, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 1539, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 2, "ArgumentList": 4112}}, "nested_types": {}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4670, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4117, "FieldOffset": 32, "Name": "FreePtr"}}, "type_references": [{"index": 4117, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4116, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4116, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 3, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 2, "ArgumentList": 4115}}, "nested_types": {}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4670, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4120, "FieldOffset": 40, "Name": "ReallocPtr"}}, "type_references": [{"index": 4120, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4119, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4119, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 1539, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 3, "ArgumentList": 4118}}, "nested_types": {}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4670, "order": 4, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4114, "FieldOffset": 48, "Name": "MallocZeroPtr"}}, "type_references": [{"index": 4114, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4113, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4113, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 1539, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 2, "ArgumentList": 4112}}, "nested_types": {}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4670, "order": 5, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4123, "FieldOffset": 56, "Name": "CallocPtr"}}, "type_references": [{"index": 4123, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4122, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4122, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 1539, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 3, "ArgumentList": 4121}}, "nested_types": {}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4670, "order": 6, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4128, "FieldOffset": 64, "Name": "DupPtr"}}, "type_references": [{"index": 4128, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4127, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4127, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 1539, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 3, "ArgumentList": 4126}}, "nested_types": {}}}}]}
// [PDB UDT 源码行] {"ipi_index": 106603, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lcms2-sys-4.0.1/vendor/include/lcms2_plugin.h", "line": 238, "module": null, "source_string_id": 74842}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] cmsPluginMutex; TPI 0x1281
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsPluginMutex", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [4740]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] cmsPluginMutex; TPI 0x1284
// 原始定义文件映射: PDB_UDT_source_line
// size = 56; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 5, "Options": ["None"], "FieldList": 4739, "Name": "cmsPluginMutex", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 56}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4739, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4294, "FieldOffset": 0, "Name": "base"}}, "type_references": [{"index": 4294, "kind": "LF_STRUCTURE", "spelling": "_cmsPluginBaseStruct", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4739, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4733, "FieldOffset": 24, "Name": "CreateMutexPtr"}}, "type_references": [{"index": 4733, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4426, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4426, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 1539, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 1, "ArgumentList": 4425}}, "nested_types": {}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4739, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4117, "FieldOffset": 32, "Name": "DestroyMutexPtr"}}, "type_references": [{"index": 4117, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4116, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4116, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 3, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 2, "ArgumentList": 4115}}, "nested_types": {}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4739, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4734, "FieldOffset": 40, "Name": "LockMutexPtr"}}, "type_references": [{"index": 4734, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4731, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4731, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 116, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 2, "ArgumentList": 4115}}, "nested_types": {}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4739, "order": 4, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4117, "FieldOffset": 48, "Name": "UnlockMutexPtr"}}, "type_references": [{"index": 4117, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4116, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4116, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 3, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 2, "ArgumentList": 4115}}, "nested_types": {}}}}]}
// [PDB UDT 源码行] {"ipi_index": 106650, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lcms2-sys-4.0.1/vendor/include/lcms2_plugin.h", "line": 658, "module": null, "source_string_id": 74842}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] cmsPluginParalellization; TPI 0x1286
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsPluginParalellization", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [4745]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] cmsPluginParalellization; TPI 0x1289
// 原始定义文件映射: PDB_UDT_source_line
// size = 40; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 4744, "Name": "cmsPluginParalellization", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 40}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4744, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4294, "FieldOffset": 0, "Name": "base"}}, "type_references": [{"index": 4294, "kind": "LF_STRUCTURE", "spelling": "_cmsPluginBaseStruct", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4744, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 116, "FieldOffset": 24, "Name": "MaxWorkers"}}, "type_references": [{"index": 116, "spelling": "Int32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4744, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 28, "Name": "WorkerFlags"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4744, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4162, "FieldOffset": 32, "Name": "SchedulerFn"}}, "type_references": [{"index": 4162, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4161, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4161, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 3, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 6, "ArgumentList": 4160}}, "nested_types": {}}}}]}
// [PDB UDT 源码行] {"ipi_index": 106657, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lcms2-sys-4.0.1/vendor/include/lcms2_plugin.h", "line": 683, "module": null, "source_string_id": 74842}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] _cmsTagBase; TPI 0x12ca
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cmsTagBase", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [4813]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] _cmsTagBase; TPI 0x12cd
// 原始定义文件映射: PDB_UDT_source_line
// size = 8; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 4812, "Name": "_cmsTagBase", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 8}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4812, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4393, "FieldOffset": 0, "Name": "sig"}}, "type_references": [{"index": 4393, "kind": "LF_ENUM", "spelling": "<unnamed-tag>", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4812, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4811, "FieldOffset": 4, "Name": "reserved"}}, "type_references": [{"index": 4811, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 16, "IndexType": 35, "Size": 4, "Name": ""}}, "nested_types": {"ElementType": {"index": 16, "spelling": "SignedCharacter", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [PDB UDT 源码行] {"ipi_index": 106700, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lcms2-sys-4.0.1/vendor/include/lcms2_plugin.h", "line": 162, "module": null, "source_string_id": 74842}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] _cmsStageToneCurvesData; TPI 0x12fe
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cmsStageToneCurvesData", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [4865]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] _cmsStageToneCurvesData; TPI 0x1301
// 原始定义文件映射: PDB_UDT_source_line
// size = 16; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 4864, "Name": "_cmsStageToneCurvesData", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 16}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4864, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 0, "Name": "nCurves"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4864, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4859, "FieldOffset": 8, "Name": "TheCurves"}}, "type_references": [{"index": 4859, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4540, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4540, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4539, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4539, "kind": "LF_STRUCTURE", "spelling": "_cms_curve_struct", "status": "PDB_record_name"}}}}}]}
// [PDB UDT 源码行] {"ipi_index": 106739, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lcms2-sys-4.0.1/vendor/include/lcms2_plugin.h", "line": 516, "module": null, "source_string_id": 74842}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] _cmsStageMatrixData; TPI 0x1306
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cmsStageMatrixData", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [4873]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] _cmsStageMatrixData; TPI 0x1309
// 原始定义文件映射: PDB_UDT_source_line
// size = 16; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 4872, "Name": "_cmsStageMatrixData", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 16}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4872, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1601, "FieldOffset": 0, "Name": "Double"}}, "type_references": [{"index": 1601, "spelling": "Float64 / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4872, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1601, "FieldOffset": 8, "Name": "Offset"}}, "type_references": [{"index": 1601, "spelling": "Float64 / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 106746, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lcms2-sys-4.0.1/vendor/include/lcms2_plugin.h", "line": 523, "module": null, "source_string_id": 74842}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] _cmsStageCLutData; TPI 0x130c
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cmsStageCLutData", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [4881]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] _cmsStageCLutData::<unnamed-tag>; TPI 0x130f
// 原始定义文件映射: PDB_UDT_source_line
// size = 8; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_UNION", "Union": {"MemberCount": 2, "Options": ["None", "Nested", "Sealed"], "FieldList": 4878, "Name": "_cmsStageCLutData::<unnamed-tag>", "UniqueName": "", "Size": 8}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4878, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1569, "FieldOffset": 0, "Name": "T"}}, "type_references": [{"index": 1569, "spelling": "UInt16Short / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4878, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1600, "FieldOffset": 0, "Name": "TFloat"}}, "type_references": [{"index": 1600, "spelling": "Float32 / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 106751, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lcms2-sys-4.0.1/vendor/include/lcms2_plugin.h", "line": 532, "module": null, "source_string_id": 74842}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] _cmsStageCLutData; TPI 0x1311
// 原始定义文件映射: PDB_UDT_source_line
// size = 24; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 5, "Options": ["None", "ContainsNestedClass"], "FieldList": 4880, "Name": "_cmsStageCLutData", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 24}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4880, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4879, "FieldOffset": 0, "Name": "Tab"}}, "type_references": [{"index": 4879, "kind": "LF_UNION", "spelling": "_cmsStageCLutData::<unnamed-tag>", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4880, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4543, "FieldOffset": 8, "Name": "Params"}}, "type_references": [{"index": 4543, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4542, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4542, "kind": "LF_STRUCTURE", "spelling": "_cms_interp_struc", "status": "PDB_record_name"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4880, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 16, "Name": "nEntries"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4880, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 116, "FieldOffset": 20, "Name": "HasFloatValues"}}, "type_references": [{"index": 116, "spelling": "Int32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4880, "order": 4, "kind": "LF_NESTTYPE", "record": {"Kind": "LF_NESTTYPE", "NestedType": {"Type": 4879, "Name": ""}}, "type_references": [{"index": 4879, "kind": "LF_UNION", "spelling": "_cmsStageCLutData::<unnamed-tag>", "status": "PDB_record_name"}]}
// [PDB UDT 源码行] {"ipi_index": 106752, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lcms2-sys-4.0.1/vendor/include/lcms2_plugin.h", "line": 530, "module": null, "source_string_id": 74842}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] cmsFormatter; TPI 0x138b
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_UNION", "Union": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsFormatter", "UniqueName": "", "Size": 0}}
// 前向定义候选（仅名称关联）: [5007]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] cmsFormatter; TPI 0x138f
// 原始定义文件映射: PDB_UDT_source_line
// size = 8; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_UNION", "Union": {"MemberCount": 2, "Options": ["None", "Sealed"], "FieldList": 5006, "Name": "cmsFormatter", "UniqueName": "", "Size": 8}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5006, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4165, "FieldOffset": 0, "Name": "Fmt16"}}, "type_references": [{"index": 4165, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4164, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4164, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 1568, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 4, "ArgumentList": 4163}}, "nested_types": {}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5006, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4168, "FieldOffset": 0, "Name": "FmtFloat"}}, "type_references": [{"index": 4168, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4167, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4167, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 1568, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 4, "ArgumentList": 4166}}, "nested_types": {}}}}]}
// [PDB UDT 源码行] {"ipi_index": 106851, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lcms2-sys-4.0.1/vendor/include/lcms2_plugin.h", "line": 361, "module": null, "source_string_id": 74842}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] <unnamed-tag>; TPI 0x139a
// 原始定义文件映射: PDB_UDT_source_line
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 2, "Options": ["None"], "FieldList": 5017, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5017, "order": 0, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "cmsFormatterInput"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5017, "order": 1, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "cmsFormatterOutput"}}, "type_references": []}
// [PDB UDT 源码行] {"ipi_index": 106861, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lcms2-sys-4.0.1/vendor/include/lcms2_plugin.h", "line": 370, "module": null, "source_string_id": 74842}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] cmsPluginFormatters; TPI 0x13a1
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsPluginFormatters", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [5028]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] cmsPluginFormatters; TPI 0x13a4
// 原始定义文件映射: PDB_UDT_source_line
// size = 32; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 5027, "Name": "cmsPluginFormatters", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 32}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5027, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4294, "FieldOffset": 0, "Name": "base"}}, "type_references": [{"index": 4294, "kind": "LF_STRUCTURE", "spelling": "_cmsPluginBaseStruct", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5027, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5021, "FieldOffset": 24, "Name": "FormattersFactory"}}, "type_references": [{"index": 5021, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 5020, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 5020, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 5003, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 3, "ArgumentList": 5019}}, "nested_types": {}}}}]}
// [PDB UDT 源码行] {"ipi_index": 106865, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lcms2-sys-4.0.1/vendor/include/lcms2_plugin.h", "line": 377, "module": null, "source_string_id": 74842}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] cmsPluginRenderingIntent; TPI 0x141c
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsPluginRenderingIntent", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [5151]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] cmsPluginRenderingIntent; TPI 0x141f
// 原始定义文件映射: PDB_UDT_source_line
// size = 296; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 5150, "Name": "cmsPluginRenderingIntent", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 296}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5150, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4294, "FieldOffset": 0, "Name": "base"}}, "type_references": [{"index": 4294, "kind": "LF_STRUCTURE", "spelling": "_cmsPluginBaseStruct", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5150, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 24, "Name": "Intent"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5150, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5124, "FieldOffset": 32, "Name": "Link"}}, "type_references": [{"index": 5124, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 5123, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 5123, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 4171, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 7, "ArgumentList": 5122}}, "nested_types": {}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5150, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4227, "FieldOffset": 40, "Name": "Description"}}, "type_references": [{"index": 4227, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 112, "IndexType": 35, "Size": 256, "Name": ""}}, "nested_types": {"ElementType": {"index": 112, "spelling": "NarrowCharacter", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [PDB UDT 源码行] {"ipi_index": 107074, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lcms2-sys-4.0.1/vendor/include/lcms2_plugin.h", "line": 469, "module": null, "source_string_id": 74842}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] cmsPluginOptimization; TPI 0x1465
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsPluginOptimization", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [5224]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] cmsPluginOptimization; TPI 0x1468
// 原始定义文件映射: PDB_UDT_source_line
// size = 32; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 5223, "Name": "cmsPluginOptimization", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 32}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5223, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4294, "FieldOffset": 0, "Name": "base"}}, "type_references": [{"index": 4294, "kind": "LF_STRUCTURE", "spelling": "_cmsPluginBaseStruct", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5223, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5217, "FieldOffset": 24, "Name": "OptimizePtr"}}, "type_references": [{"index": 5217, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 5216, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 5216, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 116, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 5, "ArgumentList": 5215}}, "nested_types": {}}}}]}
// [PDB UDT 源码行] {"ipi_index": 107132, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lcms2-sys-4.0.1/vendor/include/lcms2_plugin.h", "line": 577, "module": null, "source_string_id": 74842}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] cmsPluginTagType; TPI 0x14d9
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsPluginTagType", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [5340]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] cmsPluginTagType; TPI 0x14dc
// 原始定义文件映射: PDB_UDT_source_line
// size = 80; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 5339, "Name": "cmsPluginTagType", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 80}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5339, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4294, "FieldOffset": 0, "Name": "base"}}, "type_references": [{"index": 4294, "kind": "LF_STRUCTURE", "spelling": "_cmsPluginBaseStruct", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5339, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4369, "FieldOffset": 24, "Name": "Handler"}}, "type_references": [{"index": 4369, "kind": "LF_STRUCTURE", "spelling": "_cms_typehandler_struct", "status": "PDB_record_name"}]}
// [PDB UDT 源码行] {"ipi_index": 107249, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lcms2-sys-4.0.1/vendor/include/lcms2_plugin.h", "line": 419, "module": null, "source_string_id": 74842}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] cmsPluginTag; TPI 0x14e7
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsPluginTag", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [5354]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] cmsPluginTag; TPI 0x14ea
// 原始定义文件映射: PDB_UDT_source_line
// size = 128; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None"], "FieldList": 5353, "Name": "cmsPluginTag", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 128}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5353, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4294, "FieldOffset": 0, "Name": "base"}}, "type_references": [{"index": 4294, "kind": "LF_STRUCTURE", "spelling": "_cmsPluginBaseStruct", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5353, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4359, "FieldOffset": 24, "Name": "Signature"}}, "type_references": [{"index": 4359, "kind": "LF_ENUM", "spelling": "<unnamed-tag>", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5353, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4412, "FieldOffset": 32, "Name": "Descriptor"}}, "type_references": [{"index": 4412, "kind": "LF_STRUCTURE", "spelling": "cmsTagDescriptor", "status": "PDB_record_name"}]}
// [PDB UDT 源码行] {"ipi_index": 107256, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lcms2-sys-4.0.1/vendor/include/lcms2_plugin.h", "line": 444, "module": null, "source_string_id": 74842}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] cmsPluginParametricCurves; TPI 0x1573
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsPluginParametricCurves", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [5494]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] cmsPluginParametricCurves; TPI 0x1576
// 原始定义文件映射: PDB_UDT_source_line
// size = 200; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 5, "Options": ["None"], "FieldList": 5493, "Name": "cmsPluginParametricCurves", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 200}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5493, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4294, "FieldOffset": 0, "Name": "base"}}, "type_references": [{"index": 4294, "kind": "LF_STRUCTURE", "spelling": "_cmsPluginBaseStruct", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5493, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 24, "Name": "nFunctions"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5493, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5467, "FieldOffset": 28, "Name": "FunctionTypes"}}, "type_references": [{"index": 5467, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 117, "IndexType": 35, "Size": 80, "Name": ""}}, "nested_types": {"ElementType": {"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5493, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5467, "FieldOffset": 108, "Name": "ParameterCount"}}, "type_references": [{"index": 5467, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 117, "IndexType": 35, "Size": 80, "Name": ""}}, "nested_types": {"ElementType": {"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5493, "order": 4, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4551, "FieldOffset": 192, "Name": "Evaluator"}}, "type_references": [{"index": 4551, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4550, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4550, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 65, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 3, "ArgumentList": 4549}}, "nested_types": {}}}}]}
// [PDB UDT 源码行] {"ipi_index": 107869, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lcms2-sys-4.0.1/vendor/include/lcms2_plugin.h", "line": 332, "module": null, "source_string_id": 74842}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] cmsPluginInterpolation; TPI 0x1ac0
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsPluginInterpolation", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [6851]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] cmsPluginInterpolation; TPI 0x1ac3
// 原始定义文件映射: PDB_UDT_source_line
// size = 32; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 6850, "Name": "cmsPluginInterpolation", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 32}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6850, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4294, "FieldOffset": 0, "Name": "base"}}, "type_references": [{"index": 4294, "kind": "LF_STRUCTURE", "spelling": "_cmsPluginBaseStruct", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6850, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6845, "FieldOffset": 24, "Name": "InterpolatorsFactory"}}, "type_references": [{"index": 6845, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 6839, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 6839, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 4556, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 3, "ArgumentList": 5293}}, "nested_types": {}}}}]}
// [PDB UDT 源码行] {"ipi_index": 125043, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lcms2-sys-4.0.1/vendor/include/lcms2_plugin.h", "line": 316, "module": null, "source_string_id": 74842}
// === 完整 TPI 结构关系引用记录；原文不转换成猜测的 Rust ABI ===
// [TPI 0x1002] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cmsContext_struct", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1003] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4098, "Attrs": 65548}}
// [TPI 0x1010] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [4099, 117]}}
// [TPI 0x1011] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 1539, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 2, "ArgumentList": 4112}}
// [TPI 0x1012] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4113, "Attrs": 65548}}
// [TPI 0x1013] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [4099, 1539]}}
// [TPI 0x1014] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 3, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 2, "ArgumentList": 4115}}
// [TPI 0x1015] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4116, "Attrs": 65548}}
// [TPI 0x1016] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [4099, 1539, 117]}}
// [TPI 0x1017] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 1539, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 3, "ArgumentList": 4118}}
// [TPI 0x1018] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4119, "Attrs": 65548}}
// [TPI 0x1019] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [4099, 117, 117]}}
// [TPI 0x101a] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 1539, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 3, "ArgumentList": 4121}}
// [TPI 0x101b] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4122, "Attrs": 65548}}
// [TPI 0x101c] {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 3, "Modifiers": ["None", "Const"]}}
// [TPI 0x101d] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4124, "Attrs": 65548}}
// [TPI 0x101e] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [4099, 4125, 117]}}
// [TPI 0x101f] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 1539, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 3, "ArgumentList": 4126}}
// [TPI 0x1020] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4127, "Attrs": 65548}}
// [TPI 0x1025] {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 33, "Modifiers": ["None", "Const"]}}
// [TPI 0x1026] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4133, "Attrs": 65548}}
// [TPI 0x1031] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 1539, "Attrs": 65548}}
// [TPI 0x1039] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cmstransform_struct", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x103a] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4153, "Attrs": 65548}}
// [TPI 0x103d] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsStride", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x103e] {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 4157, "Modifiers": ["None", "Const"]}}
// [TPI 0x103f] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4158, "Attrs": 65548}}
// [TPI 0x1040] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [4154, 4125, 1539, 117, 117, 4159]}}
// [TPI 0x1041] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 3, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 6, "ArgumentList": 4160}}
// [TPI 0x1042] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4161, "Attrs": 65548}}
// [TPI 0x1043] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [4154, 1569, 1568, 117]}}
// [TPI 0x1044] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 1568, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 4, "ArgumentList": 4163}}
// [TPI 0x1045] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4164, "Attrs": 65548}}
// [TPI 0x1046] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [4154, 1600, 1568, 117]}}
// [TPI 0x1047] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 1568, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 4, "ArgumentList": 4166}}
// [TPI 0x1048] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4167, "Attrs": 65548}}
// [TPI 0x104a] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cmsPipeline_struct", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x104b] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4170, "Attrs": 65548}}
// [TPI 0x1051] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [4154, 4125, 1539, 117, 117]}}
// [TPI 0x1052] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 3, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 5, "ArgumentList": 4177}}
// [TPI 0x1053] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4178, "Attrs": 65548}}
// [TPI 0x1056] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 0, "Name": "BytesPerLineIn"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 4, "Name": "BytesPerLineOut"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 8, "Name": "BytesPerPlaneIn"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 12, "Name": "BytesPerPlaneOut"}}]}
// [TPI 0x1057] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 4182, "Name": "cmsStride", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 16}}
// [TPI 0x1060] {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 64, "Modifiers": ["None", "Const"]}}
// [TPI 0x1061] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4192, "Attrs": 65548}}
// [TPI 0x1083] {"Kind": "LF_ARRAY", "Array": {"ElementType": 112, "IndexType": 35, "Size": 256, "Name": ""}}
// [TPI 0x10bd] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4162, "Attrs": 65548}}
// [TPI 0x10be] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4117, "Attrs": 65548}}
// [TPI 0x10bf] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4171, "Attrs": 65548}}
// [TPI 0x10c0] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [4285, 4145, 4286, 4287, 1653, 1653, 1653]}}
// [TPI 0x10c1] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 116, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 7, "ArgumentList": 4288}}
// [TPI 0x10c2] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4289, "Attrs": 65548}}
// [TPI 0x10c6] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cmsPluginBaseStruct", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x10c7] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4294, "Attrs": 65548}}
// [TPI 0x10ca] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 0, "Name": "Magic"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 4, "Name": "ExpectedVersion"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 8, "Name": "Type"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4295, "FieldOffset": 16, "Name": "Next"}}]}
// [TPI 0x10cb] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 4298, "Name": "_cmsPluginBaseStruct", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 24}}
// [TPI 0x10cc] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsPluginTransform", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x10ce] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4179, "Attrs": 65548}}
// [TPI 0x10cf] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [4302, 4145, 4286, 4287, 1653, 1653, 1653]}}
// [TPI 0x10d0] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 116, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 7, "ArgumentList": 4303}}
// [TPI 0x10d1] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4304, "Attrs": 65548}}
// [TPI 0x10d2] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4305, "FieldOffset": 0, "Name": "legacy_xform"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4290, "FieldOffset": 0, "Name": "xform"}}]}
// [TPI 0x10d3] {"Kind": "LF_UNION", "Union": {"MemberCount": 2, "Options": ["None", "Nested", "Sealed"], "FieldList": 4306, "Name": "cmsPluginTransform::<unnamed-tag>", "UniqueName": "", "Size": 8}}
// [TPI 0x10d4] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4294, "FieldOffset": 0, "Name": "base"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4307, "FieldOffset": 24, "Name": "factories"}}, {"Kind": "LF_NESTTYPE", "NestedType": {"Type": 4307, "Name": ""}}]}
// [TPI 0x10d5] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "ContainsNestedClass"], "FieldList": 4308, "Name": "cmsPluginTransform", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 32}}
// [TPI 0x1106] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1093812784, "Name": "cmsSigAToB0Tag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1093812785, "Name": "cmsSigAToB1Tag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1093812786, "Name": "cmsSigAToB2Tag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1649957210, "Name": "cmsSigBlueColorantTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1649957210, "Name": "cmsSigBlueMatrixColumnTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1649693251, "Name": "cmsSigBlueTRCTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1110589744, "Name": "cmsSigBToA0Tag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1110589745, "Name": "cmsSigBToA1Tag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1110589746, "Name": "cmsSigBToA2Tag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1667329140, "Name": "cmsSigCalibrationDateTimeTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1952543335, "Name": "cmsSigCharTargetTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1667785060, "Name": "cmsSigChromaticAdaptationTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1667789421, "Name": "cmsSigChromaticityTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1668051567, "Name": "cmsSigColorantOrderTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1668051572, "Name": "cmsSigColorantTableTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1668050804, "Name": "cmsSigColorantTableOutTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1667852659, "Name": "cmsSigColorimetricIntentImageStateTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1668313716, "Name": "cmsSigCopyrightTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1668441193, "Name": "cmsSigCrdInfoTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1684108385, "Name": "cmsSigDataTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1685350765, "Name": "cmsSigDateTimeTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1684893284, "Name": "cmsSigDeviceMfgDescTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1684890724, "Name": "cmsSigDeviceModelDescTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1684371059, "Name": "cmsSigDeviceSettingsTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1144144432, "Name": "cmsSigDToB0Tag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1144144433, "Name": "cmsSigDToB1Tag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1144144434, "Name": "cmsSigDToB2Tag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1144144435, "Name": "cmsSigDToB3Tag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1110590512, "Name": "cmsSigBToD0Tag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1110590513, "Name": "cmsSigBToD1Tag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1110590514, "Name": "cmsSigBToD2Tag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1110590515, "Name": "cmsSigBToD3Tag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1734438260, "Name": "cmsSigGamutTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1800688195, "Name": "cmsSigGrayTRCTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1733843290, "Name": "cmsSigGreenColorantTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1733843290, "Name": "cmsSigGreenMatrixColumnTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1733579331, "Name": "cmsSigGreenTRCTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1819635049, "Name": "cmsSigLuminanceTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1835360627, "Name": "cmsSigMeasurementTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1651208308, "Name": "cmsSigMediaBlackPointTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 2004119668, "Name": "cmsSigMediaWhitePointTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1852010348, "Name": "cmsSigNamedColorTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1852009522, "Name": "cmsSigNamedColor2Tag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1919251312, "Name": "cmsSigOutputResponseTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1919510320, "Name": "cmsSigPerceptualRenderingIntentGamutTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1886545200, "Name": "cmsSigPreview0Tag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1886545201, "Name": "cmsSigPreview1Tag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1886545202, "Name": "cmsSigPreview2Tag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1684370275, "Name": "cmsSigProfileDescriptionTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1685283693, "Name": "cmsSigProfileDescriptionMLTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1886610801, "Name": "cmsSigProfileSequenceDescTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1886611812, "Name": "cmsSigProfileSequenceIdTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1886610480, "Name": "cmsSigPs2CRD0Tag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1886610481, "Name": "cmsSigPs2CRD1Tag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1886610482, "Name": "cmsSigPs2CRD2Tag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1886610483, "Name": "cmsSigPs2CRD3Tag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1886597747, "Name": "cmsSigPs2CSATag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1886597737, "Name": "cmsSigPs2RenderingIntentTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1918392666, "Name": "cmsSigRedColorantTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1918392666, "Name": "cmsSigRedMatrixColumnTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1918128707, "Name": "cmsSigRedTRCTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1919510322, "Name": "cmsSigSaturationRenderingIntentGamutTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1935897188, "Name": "cmsSigScreeningDescTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1935897198, "Name": "cmsSigScreeningTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1952801640, "Name": "cmsSigTechnologyTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1650877472, "Name": "cmsSigUcrBgTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1987405156, "Name": "cmsSigViewingCondDescTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1986618743, "Name": "cmsSigViewingConditionsTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1986226036, "Name": "cmsSigVcgtTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1835365473, "Name": "cmsSigMetaTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1667851120, "Name": "cmsSigcicpTag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1634890867, "Name": "cmsSigArgyllArtsTag"}}]}
// [TPI 0x1107] {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 72, "Options": ["None"], "FieldList": 4358, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [TPI 0x110a] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cms_io_handler", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x110b] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4362, "Attrs": 65548}}
// [TPI 0x1111] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cms_typehandler_struct", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1112] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4369, "Attrs": 65548}}
// [TPI 0x1116] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [4363, 1539, 117, 117]}}
// [TPI 0x1117] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 117, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 4, "ArgumentList": 4374}}
// [TPI 0x1118] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4375, "Attrs": 65548}}
// [TPI 0x1119] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [4363, 117]}}
// [TPI 0x111a] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 116, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 2, "ArgumentList": 4377}}
// [TPI 0x111b] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4378, "Attrs": 65548}}
// [TPI 0x111c] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [4363]}}
// [TPI 0x111d] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 116, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 1, "ArgumentList": 4380}}
// [TPI 0x111e] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4381, "Attrs": 65548}}
// [TPI 0x111f] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 117, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 1, "ArgumentList": 4380}}
// [TPI 0x1120] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4383, "Attrs": 65548}}
// [TPI 0x1121] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [4363, 117, 4125]}}
// [TPI 0x1122] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 116, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 3, "ArgumentList": 4385}}
// [TPI 0x1123] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4386, "Attrs": 65548}}
// [TPI 0x1124] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 0, "Name": "stream"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4099, "FieldOffset": 8, "Name": "ContextID"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 16, "Name": "UsedSpace"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 20, "Name": "ReportedSize"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4227, "FieldOffset": 24, "Name": "PhysicalFile"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4376, "FieldOffset": 280, "Name": "Read"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4379, "FieldOffset": 288, "Name": "Seek"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4382, "FieldOffset": 296, "Name": "Close"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4384, "FieldOffset": 304, "Name": "Tell"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4387, "FieldOffset": 312, "Name": "Write"}}]}
// [TPI 0x1125] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 10, "Options": ["None"], "FieldList": 4388, "Name": "_cms_io_handler", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 320}}
// [TPI 0x1128] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1667789421, "Name": "cmsSigChromaticityType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1667851120, "Name": "cmsSigcicpType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1668051567, "Name": "cmsSigColorantOrderType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1668051572, "Name": "cmsSigColorantTableType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1668441193, "Name": "cmsSigCrdInfoType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1668641398, "Name": "cmsSigCurveType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1684108385, "Name": "cmsSigDataType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1684628340, "Name": "cmsSigDictType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1685350765, "Name": "cmsSigDateTimeType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1684371059, "Name": "cmsSigDeviceSettingsType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1835430962, "Name": "cmsSigLut16Type"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1835430961, "Name": "cmsSigLut8Type"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1832993312, "Name": "cmsSigLutAtoBType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1833058592, "Name": "cmsSigLutBtoAType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1835360627, "Name": "cmsSigMeasurementType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1835824483, "Name": "cmsSigMultiLocalizedUnicodeType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1836082548, "Name": "cmsSigMultiProcessElementType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1852010348, "Name": "cmsSigNamedColorType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1852009522, "Name": "cmsSigNamedColor2Type"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1885434465, "Name": "cmsSigParametricCurveType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1886610801, "Name": "cmsSigProfileSequenceDescType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1886611812, "Name": "cmsSigProfileSequenceIdType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1919120178, "Name": "cmsSigResponseCurveSet16Type"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1936077618, "Name": "cmsSigS15Fixed16ArrayType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1935897198, "Name": "cmsSigScreeningType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1936287520, "Name": "cmsSigSignatureType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1952807028, "Name": "cmsSigTextType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1684370275, "Name": "cmsSigTextDescriptionType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1969632050, "Name": "cmsSigU16Fixed16ArrayType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1650877472, "Name": "cmsSigUcrBgType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1969828150, "Name": "cmsSigUInt16ArrayType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1969828658, "Name": "cmsSigUInt32ArrayType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1969829428, "Name": "cmsSigUInt64ArrayType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1969827896, "Name": "cmsSigUInt8ArrayType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1986226036, "Name": "cmsSigVcgtType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1986618743, "Name": "cmsSigViewingConditionsType"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1482250784, "Name": "cmsSigXYZType"}}]}
// [TPI 0x1129] {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 37, "Options": ["None"], "FieldList": 4392, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [TPI 0x112a] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [4370, 4363, 1653, 117]}}
// [TPI 0x112b] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 1539, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 4, "ArgumentList": 4394}}
// [TPI 0x112c] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4395, "Attrs": 65548}}
// [TPI 0x112d] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [4370, 4363, 1539, 117]}}
// [TPI 0x112e] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 116, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 4, "ArgumentList": 4397}}
// [TPI 0x112f] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4398, "Attrs": 65548}}
// [TPI 0x1130] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [4370, 4125, 117]}}
// [TPI 0x1131] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 1539, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 3, "ArgumentList": 4400}}
// [TPI 0x1132] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4401, "Attrs": 65548}}
// [TPI 0x1133] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [4370, 1539]}}
// [TPI 0x1134] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 3, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 2, "ArgumentList": 4403}}
// [TPI 0x1135] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4404, "Attrs": 65548}}
// [TPI 0x1136] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4393, "FieldOffset": 0, "Name": "Signature"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4396, "FieldOffset": 8, "Name": "ReadPtr"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4399, "FieldOffset": 16, "Name": "WritePtr"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4402, "FieldOffset": 24, "Name": "DupPtr"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4405, "FieldOffset": 32, "Name": "FreePtr"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4099, "FieldOffset": 40, "Name": "ContextID"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 48, "Name": "ICCVersion"}}]}
// [TPI 0x1137] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 7, "Options": ["None"], "FieldList": 4406, "Name": "_cms_typehandler_struct", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 56}}
// [TPI 0x113c] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsTagDescriptor", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1141] {"Kind": "LF_ARRAY", "Array": {"ElementType": 4393, "IndexType": 35, "Size": 80, "Name": ""}}
// [TPI 0x1142] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [65, 4125]}}
// [TPI 0x1143] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 4393, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 2, "ArgumentList": 4418}}
// [TPI 0x1144] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4419, "Attrs": 65548}}
// [TPI 0x1145] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 0, "Name": "ElemCount"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 4, "Name": "nSupportedTypes"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4417, "FieldOffset": 8, "Name": "SupportedTypes"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4420, "FieldOffset": 88, "Name": "DecideType"}}]}
// [TPI 0x1146] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 4421, "Name": "cmsTagDescriptor", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 96}}
// [TPI 0x1149] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [4099]}}
// [TPI 0x114a] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 1539, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 1, "ArgumentList": 4425}}
// [TPI 0x11bb] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cms_curve_struct", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x11bc] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4539, "Attrs": 65548}}
// [TPI 0x11be] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cms_interp_struc", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x11bf] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4542, "Attrs": 65548}}
// [TPI 0x11c3] {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 65, "Modifiers": ["None", "Const"]}}
// [TPI 0x11c4] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4547, "Attrs": 65548}}
// [TPI 0x11c5] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [116, 4548, 65]}}
// [TPI 0x11c6] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 65, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 3, "ArgumentList": 4549}}
// [TPI 0x11c7] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4550, "Attrs": 65548}}
// [TPI 0x11cb] {"Kind": "LF_ARRAY", "Array": {"ElementType": 117, "IndexType": 35, "Size": 60, "Name": ""}}
// [TPI 0x11cc] {"Kind": "LF_UNION", "Union": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsInterpFunction", "UniqueName": "", "Size": 0}}
// [TPI 0x11cd] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4099, "FieldOffset": 0, "Name": "ContextID"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 8, "Name": "dwFlags"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 12, "Name": "nInputs"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 16, "Name": "nOutputs"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4555, "FieldOffset": 20, "Name": "nSamples"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4555, "FieldOffset": 80, "Name": "Domain"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4555, "FieldOffset": 140, "Name": "opta"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4125, "FieldOffset": 200, "Name": "Table"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4556, "FieldOffset": 208, "Name": "Interpolation"}}]}
// [TPI 0x11ce] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 9, "Options": ["None"], "FieldList": 4557, "Name": "_cms_interp_struc", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 216}}
// [TPI 0x11d2] {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 4542, "Modifiers": ["None", "Const"]}}
// [TPI 0x11d3] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4562, "Attrs": 65548}}
// [TPI 0x11d4] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [4134, 1569, 4563]}}
// [TPI 0x11d5] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 3, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 3, "ArgumentList": 4564}}
// [TPI 0x11d6] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4565, "Attrs": 65548}}
// [TPI 0x11d7] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [4193, 1600, 4563]}}
// [TPI 0x11d8] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 3, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 3, "ArgumentList": 4567}}
// [TPI 0x11d9] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4568, "Attrs": 65548}}
// [TPI 0x11da] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4566, "FieldOffset": 0, "Name": "Lerp16"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4569, "FieldOffset": 0, "Name": "LerpFloat"}}]}
// [TPI 0x11db] {"Kind": "LF_UNION", "Union": {"MemberCount": 2, "Options": ["None", "Sealed"], "FieldList": 4570, "Name": "cmsInterpFunction", "UniqueName": "", "Size": 8}}
// [TPI 0x11f9] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsMAT3", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x11fa] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsVEC3", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x11fb] {"Kind": "LF_ARRAY", "Array": {"ElementType": 4602, "IndexType": 35, "Size": 72, "Name": ""}}
// [TPI 0x11fc] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4603, "FieldOffset": 0, "Name": "v"}}]}
// [TPI 0x11fd] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 1, "Options": ["None"], "FieldList": 4604, "Name": "cmsMAT3", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 72}}
// [TPI 0x11fe] {"Kind": "LF_ARRAY", "Array": {"ElementType": 65, "IndexType": 35, "Size": 24, "Name": ""}}
// [TPI 0x11ff] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4606, "FieldOffset": 0, "Name": "n"}}]}
// [TPI 0x1200] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 1, "Options": ["None"], "FieldList": 4607, "Name": "cmsVEC3", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 24}}
// [TPI 0x1239] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsPluginMemHandler", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x123e] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4294, "FieldOffset": 0, "Name": "base"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4114, "FieldOffset": 24, "Name": "MallocPtr"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4117, "FieldOffset": 32, "Name": "FreePtr"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4120, "FieldOffset": 40, "Name": "ReallocPtr"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4114, "FieldOffset": 48, "Name": "MallocZeroPtr"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4123, "FieldOffset": 56, "Name": "CallocPtr"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4128, "FieldOffset": 64, "Name": "DupPtr"}}]}
// [TPI 0x123f] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 7, "Options": ["None"], "FieldList": 4670, "Name": "cmsPluginMemHandler", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 72}}
// [TPI 0x127b] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 116, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 2, "ArgumentList": 4115}}
// [TPI 0x127d] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4426, "Attrs": 65548}}
// [TPI 0x127e] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4731, "Attrs": 65548}}
// [TPI 0x1281] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsPluginMutex", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1283] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4294, "FieldOffset": 0, "Name": "base"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4733, "FieldOffset": 24, "Name": "CreateMutexPtr"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4117, "FieldOffset": 32, "Name": "DestroyMutexPtr"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4734, "FieldOffset": 40, "Name": "LockMutexPtr"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4117, "FieldOffset": 48, "Name": "UnlockMutexPtr"}}]}
// [TPI 0x1284] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 5, "Options": ["None"], "FieldList": 4739, "Name": "cmsPluginMutex", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 56}}
// [TPI 0x1286] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsPluginParalellization", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1288] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4294, "FieldOffset": 0, "Name": "base"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 116, "FieldOffset": 24, "Name": "MaxWorkers"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 28, "Name": "WorkerFlags"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4162, "FieldOffset": 32, "Name": "SchedulerFn"}}]}
// [TPI 0x1289] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 4744, "Name": "cmsPluginParalellization", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 40}}
// [TPI 0x12ca] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cmsTagBase", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x12cb] {"Kind": "LF_ARRAY", "Array": {"ElementType": 16, "IndexType": 35, "Size": 4, "Name": ""}}
// [TPI 0x12cc] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4393, "FieldOffset": 0, "Name": "sig"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4811, "FieldOffset": 4, "Name": "reserved"}}]}
// [TPI 0x12cd] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 4812, "Name": "_cmsTagBase", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 8}}
// [TPI 0x12fb] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4540, "Attrs": 65548}}
// [TPI 0x12fe] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cmsStageToneCurvesData", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1300] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 0, "Name": "nCurves"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4859, "FieldOffset": 8, "Name": "TheCurves"}}]}
// [TPI 0x1301] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 4864, "Name": "_cmsStageToneCurvesData", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 16}}
// [TPI 0x1306] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cmsStageMatrixData", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1308] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1601, "FieldOffset": 0, "Name": "Double"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1601, "FieldOffset": 8, "Name": "Offset"}}]}
// [TPI 0x1309] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 4872, "Name": "_cmsStageMatrixData", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 16}}
// [TPI 0x130c] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cmsStageCLutData", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x130e] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1569, "FieldOffset": 0, "Name": "T"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1600, "FieldOffset": 0, "Name": "TFloat"}}]}
// [TPI 0x130f] {"Kind": "LF_UNION", "Union": {"MemberCount": 2, "Options": ["None", "Nested", "Sealed"], "FieldList": 4878, "Name": "_cmsStageCLutData::<unnamed-tag>", "UniqueName": "", "Size": 8}}
// [TPI 0x1310] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4879, "FieldOffset": 0, "Name": "Tab"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4543, "FieldOffset": 8, "Name": "Params"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 16, "Name": "nEntries"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 116, "FieldOffset": 20, "Name": "HasFloatValues"}}, {"Kind": "LF_NESTTYPE", "NestedType": {"Type": 4879, "Name": ""}}]}
// [TPI 0x1311] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 5, "Options": ["None", "ContainsNestedClass"], "FieldList": 4880, "Name": "_cmsStageCLutData", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 24}}
// [TPI 0x138b] {"Kind": "LF_UNION", "Union": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsFormatter", "UniqueName": "", "Size": 0}}
// [TPI 0x138e] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4165, "FieldOffset": 0, "Name": "Fmt16"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4168, "FieldOffset": 0, "Name": "FmtFloat"}}]}
// [TPI 0x138f] {"Kind": "LF_UNION", "Union": {"MemberCount": 2, "Options": ["None", "Sealed"], "FieldList": 5006, "Name": "cmsFormatter", "UniqueName": "", "Size": 8}}
// [TPI 0x1399] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "cmsFormatterInput"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "cmsFormatterOutput"}}]}
// [TPI 0x139a] {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 2, "Options": ["None"], "FieldList": 5017, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [TPI 0x139b] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [117, 5018, 117]}}
// [TPI 0x139c] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 5003, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 3, "ArgumentList": 5019}}
// [TPI 0x139d] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 5020, "Attrs": 65548}}
// [TPI 0x13a1] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsPluginFormatters", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x13a3] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4294, "FieldOffset": 0, "Name": "base"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5021, "FieldOffset": 24, "Name": "FormattersFactory"}}]}
// [TPI 0x13a4] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 5027, "Name": "cmsPluginFormatters", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 32}}
// [TPI 0x1402] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [4099, 117, 1653, 4145, 1652, 1601, 117]}}
// [TPI 0x1403] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 4171, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 7, "ArgumentList": 5122}}
// [TPI 0x1404] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 5123, "Attrs": 65548}}
// [TPI 0x141c] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsPluginRenderingIntent", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x141e] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4294, "FieldOffset": 0, "Name": "base"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 24, "Name": "Intent"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5124, "FieldOffset": 32, "Name": "Link"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4227, "FieldOffset": 40, "Name": "Description"}}]}
// [TPI 0x141f] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 5150, "Name": "cmsPluginRenderingIntent", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 296}}
// [TPI 0x145f] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [4287, 117, 1653, 1653, 1653]}}
// [TPI 0x1460] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 116, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 5, "ArgumentList": 5215}}
// [TPI 0x1461] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 5216, "Attrs": 65548}}
// [TPI 0x1465] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsPluginOptimization", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1467] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4294, "FieldOffset": 0, "Name": "base"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5217, "FieldOffset": 24, "Name": "OptimizePtr"}}]}
// [TPI 0x1468] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 5223, "Name": "cmsPluginOptimization", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 32}}
// [TPI 0x14ad] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [117, 117, 117]}}
// [TPI 0x14d9] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsPluginTagType", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x14db] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4294, "FieldOffset": 0, "Name": "base"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4369, "FieldOffset": 24, "Name": "Handler"}}]}
// [TPI 0x14dc] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 5339, "Name": "cmsPluginTagType", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 80}}
// [TPI 0x14e7] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsPluginTag", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x14e9] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4294, "FieldOffset": 0, "Name": "base"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4359, "FieldOffset": 24, "Name": "Signature"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4412, "FieldOffset": 32, "Name": "Descriptor"}}]}
// [TPI 0x14ea] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None"], "FieldList": 5353, "Name": "cmsPluginTag", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 128}}
// [TPI 0x155b] {"Kind": "LF_ARRAY", "Array": {"ElementType": 117, "IndexType": 35, "Size": 80, "Name": ""}}
// [TPI 0x1573] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsPluginParametricCurves", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1575] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4294, "FieldOffset": 0, "Name": "base"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 24, "Name": "nFunctions"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5467, "FieldOffset": 28, "Name": "FunctionTypes"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5467, "FieldOffset": 108, "Name": "ParameterCount"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4551, "FieldOffset": 192, "Name": "Evaluator"}}]}
// [TPI 0x1576] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 5, "Options": ["None"], "FieldList": 5493, "Name": "cmsPluginParametricCurves", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 200}}
// [TPI 0x1ab7] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 4556, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 3, "ArgumentList": 5293}}
// [TPI 0x1abd] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 6839, "Attrs": 65548}}
// [TPI 0x1ac0] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsPluginInterpolation", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1ac2] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4294, "FieldOffset": 0, "Name": "base"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6845, "FieldOffset": 24, "Name": "InterpolatorsFactory"}}]}
// [TPI 0x1ac3] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 6850, "Name": "cmsPluginInterpolation", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 32}}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: lcms2_plugin.h.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===
