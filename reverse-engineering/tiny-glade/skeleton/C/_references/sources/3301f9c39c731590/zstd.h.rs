// Generated from PDB metadata by skeleton/_tools/fill_type_structures.py.
// Version: C
// 类型元数据；原作者源码文本/定义文件见逐项映射；不生成依赖实现。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: zstd.h.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB forward_declaration_only] ZSTD_customMem; TPI 0x1239
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ZSTD_customMem", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [4701]
// ------------------------------------------------------------------
// [PDB forward_declaration_only] ZSTD_inBuffer_s; TPI 0x124e
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ZSTD_inBuffer_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [4754]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] ZSTD_customMem; TPI 0x125d
// 原始定义文件映射: PDB_UDT_source_line
// size = 24; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None"], "FieldList": 4700, "Name": "ZSTD_customMem", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 24}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4700, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4696, "FieldOffset": 0, "Name": "customAlloc"}}, "type_references": [{"index": 4696, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4695, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4695, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 1539, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 2, "ArgumentList": 4488}}, "nested_types": {}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4700, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4699, "FieldOffset": 8, "Name": "customFree"}}, "type_references": [{"index": 4699, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4698, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4698, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 3, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 2, "ArgumentList": 4697}}, "nested_types": {}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4700, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 16, "Name": "opaque"}}, "type_references": [{"index": 1539, "spelling": "Void / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 101711, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/../common/../zstd.h", "line": 1587, "module": null, "source_string_id": 101710}
// [PDB UDT 源码行] {"ipi_index": 102223, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/decompress/../common/../zstd.h", "line": 1587, "module": null, "source_string_id": 102208}
// [PDB UDT 源码行] {"ipi_index": 131239, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/common/../zstd.h", "line": 1587, "module": null, "source_string_id": 131238}
// [PDB UDT 源码行] {"ipi_index": 131353, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/../zstd.h", "line": 1587, "module": null, "source_string_id": 131345}
// [PDB UDT 源码行] {"ipi_index": 131724, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/dictBuilder/../common/../zstd.h", "line": 1587, "module": null, "source_string_id": 131702}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] <unnamed-tag>; TPI 0x125f
// 原始定义文件映射: PDB_UDT_source_line
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 2, "Options": ["None"], "FieldList": 4702, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4702, "order": 0, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "ZSTD_f_zstd1"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4702, "order": 1, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "ZSTD_f_zstd1_magicless"}}, "type_references": []}
// [PDB UDT 源码行] {"ipi_index": 101712, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/../common/../zstd.h", "line": 1247, "module": null, "source_string_id": 101710}
// [PDB UDT 源码行] {"ipi_index": 102209, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/decompress/../common/../zstd.h", "line": 1247, "module": null, "source_string_id": 102208}
// [PDB UDT 源码行] {"ipi_index": 131346, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/../zstd.h", "line": 1247, "module": null, "source_string_id": 131345}
// [PDB UDT 源码行] {"ipi_index": 131712, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/dictBuilder/../common/../zstd.h", "line": 1247, "module": null, "source_string_id": 131702}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] ZSTD_inBuffer_s; TPI 0x1292
// 原始定义文件映射: PDB_UDT_source_line
// size = 24; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None"], "FieldList": 4753, "Name": "ZSTD_inBuffer_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 24}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4753, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4125, "FieldOffset": 0, "Name": "src"}}, "type_references": [{"index": 4125, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4124, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4124, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 3, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 3, "spelling": "Void", "status": "CodeView_simple_type; not Rust source spelling"}}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4753, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 8, "Name": "size"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4753, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 16, "Name": "pos"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 101732, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/../common/../zstd.h", "line": 596, "module": null, "source_string_id": 101710}
// [PDB UDT 源码行] {"ipi_index": 102342, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/decompress/../common/../zstd.h", "line": 596, "module": null, "source_string_id": 102208}
// [PDB UDT 源码行] {"ipi_index": 131366, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/../zstd.h", "line": 596, "module": null, "source_string_id": 131345}
// [PDB UDT 源码行] {"ipi_index": 131732, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/dictBuilder/../common/../zstd.h", "line": 596, "module": null, "source_string_id": 131702}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] <unnamed-tag>; TPI 0x1294
// 原始定义文件映射: PDB_UDT_source_line
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 3, "Options": ["None"], "FieldList": 4755, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4755, "order": 0, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "ZSTD_dct_auto"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4755, "order": 1, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "ZSTD_dct_rawContent"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4755, "order": 2, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 2, "Name": "ZSTD_dct_fullDict"}}, "type_references": []}
// [PDB UDT 源码行] {"ipi_index": 101733, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/../common/../zstd.h", "line": 1236, "module": null, "source_string_id": 101710}
// [PDB UDT 源码行] {"ipi_index": 102260, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/decompress/../common/../zstd.h", "line": 1236, "module": null, "source_string_id": 102208}
// [PDB UDT 源码行] {"ipi_index": 131367, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/../zstd.h", "line": 1236, "module": null, "source_string_id": 131345}
// [PDB UDT 源码行] {"ipi_index": 131733, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/dictBuilder/../common/../zstd.h", "line": 1236, "module": null, "source_string_id": 131702}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] <unnamed-tag>; TPI 0x135d
// 原始定义文件映射: PDB_UDT_source_line
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 3, "Options": ["None"], "FieldList": 4956, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4956, "order": 0, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "ZSTD_reset_session_only"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4956, "order": 1, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 2, "Name": "ZSTD_reset_parameters"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4956, "order": 2, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 3, "Name": "ZSTD_reset_session_and_parameters"}}, "type_references": []}
// [PDB UDT 源码行] {"ipi_index": 101826, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/../common/../zstd.h", "line": 490, "module": null, "source_string_id": 101710}
// [PDB UDT 源码行] {"ipi_index": 102263, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/decompress/../common/../zstd.h", "line": 490, "module": null, "source_string_id": 102208}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] <unnamed-tag>; TPI 0x13bd
// 原始定义文件映射: PDB_UDT_source_line
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 2, "Options": ["None"], "FieldList": 5052, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5052, "order": 0, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "ZSTD_dlm_byCopy"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5052, "order": 1, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "ZSTD_dlm_byRef"}}, "type_references": []}
// [PDB UDT 源码行] {"ipi_index": 101875, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/../common/../zstd.h", "line": 1242, "module": null, "source_string_id": 101710}
// [PDB UDT 源码行] {"ipi_index": 102259, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/decompress/../common/../zstd.h", "line": 1242, "module": null, "source_string_id": 102208}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] ZSTD_outBuffer_s; TPI 0x13db
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ZSTD_outBuffer_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [5091]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] ZSTD_outBuffer_s; TPI 0x13e3
// 原始定义文件映射: PDB_UDT_source_line
// size = 24; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None"], "FieldList": 5090, "Name": "ZSTD_outBuffer_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 24}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5090, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 0, "Name": "dst"}}, "type_references": [{"index": 1539, "spelling": "Void / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5090, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 8, "Name": "size"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5090, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 16, "Name": "pos"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 101894, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/../common/../zstd.h", "line": 602, "module": null, "source_string_id": 101710}
// [PDB UDT 源码行] {"ipi_index": 102225, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/decompress/../common/../zstd.h", "line": 602, "module": null, "source_string_id": 102208}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] ZSTD_bounds; TPI 0x1477
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ZSTD_bounds", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [5242]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] ZSTD_bounds; TPI 0x147a
// 原始定义文件映射: PDB_UDT_source_line
// size = 16; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None"], "FieldList": 5241, "Name": "ZSTD_bounds", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 16}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5241, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 0, "Name": "error"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5241, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 116, "FieldOffset": 8, "Name": "lowerBound"}}, "type_references": [{"index": 116, "spelling": "Int32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5241, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 116, "FieldOffset": 12, "Name": "upperBound"}}, "type_references": [{"index": 116, "spelling": "Int32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 101975, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/../common/../zstd.h", "line": 445, "module": null, "source_string_id": 101710}
// [PDB UDT 源码行] {"ipi_index": 102336, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/decompress/../common/../zstd.h", "line": 445, "module": null, "source_string_id": 102208}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] ZSTD_frameHeader; TPI 0x1544
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ZSTD_frameHeader", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [5480]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] <unnamed-tag>; TPI 0x154a
// 原始定义文件映射: PDB_UDT_source_line
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 2, "Options": ["None"], "FieldList": 5449, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5449, "order": 0, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "ZSTD_d_validateChecksum"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5449, "order": 1, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "ZSTD_d_ignoreChecksum"}}, "type_references": []}
// [PDB UDT 源码行] {"ipi_index": 102210, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/decompress/../common/../zstd.h", "line": 1254, "module": null, "source_string_id": 102208}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] <unnamed-tag>; TPI 0x1554
// 原始定义文件映射: PDB_UDT_source_line
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 2, "Options": ["None"], "FieldList": 5459, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5459, "order": 0, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "ZSTD_rmd_refSingleDDict"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5459, "order": 1, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "ZSTD_rmd_refMultipleDDicts"}}, "type_references": []}
// [PDB UDT 源码行] {"ipi_index": 102212, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/decompress/../common/../zstd.h", "line": 1260, "module": null, "source_string_id": 102208}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] <unnamed-tag>; TPI 0x1566
// 原始定义文件映射: PDB_UDT_source_line
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 2, "Options": ["None"], "FieldList": 5477, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5477, "order": 0, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "ZSTD_frame"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5477, "order": 1, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "ZSTD_skippableFrame"}}, "type_references": []}
// [PDB UDT 源码行] {"ipi_index": 102219, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/decompress/../common/../zstd.h", "line": 2495, "module": null, "source_string_id": 102208}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] ZSTD_frameHeader; TPI 0x1568
// 原始定义文件映射: PDB_UDT_source_line
// size = 40; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 7, "Options": ["None"], "FieldList": 5479, "Name": "ZSTD_frameHeader", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 40}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5479, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 0, "Name": "frameContentSize"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5479, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 8, "Name": "windowSize"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5479, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 16, "Name": "blockSizeMax"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5479, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5478, "FieldOffset": 20, "Name": "frameType"}}, "type_references": [{"index": 5478, "kind": "LF_ENUM", "spelling": "<unnamed-tag>", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5479, "order": 4, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 24, "Name": "headerSize"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5479, "order": 5, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 28, "Name": "dictID"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5479, "order": 6, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 32, "Name": "checksumFlag"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 102220, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/decompress/../common/../zstd.h", "line": 2496, "module": null, "source_string_id": 102208}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] <unnamed-tag>; TPI 0x15a6
// 原始定义文件映射: PDB_UDT_source_line
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 5, "Options": ["None"], "FieldList": 5541, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5541, "order": 0, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 100, "Name": "ZSTD_d_windowLogMax"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5541, "order": 1, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1000, "Name": "ZSTD_d_experimentalParam1"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5541, "order": 2, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1001, "Name": "ZSTD_d_experimentalParam2"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5541, "order": 3, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1002, "Name": "ZSTD_d_experimentalParam3"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5541, "order": 4, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1003, "Name": "ZSTD_d_experimentalParam4"}}, "type_references": []}
// [PDB UDT 源码行] {"ipi_index": 102275, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/decompress/../common/../zstd.h", "line": 539, "module": null, "source_string_id": 102208}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] <unnamed-tag>; TPI 0x15dd
// 原始定义文件映射: PDB_UDT_source_line
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 6, "Options": ["None"], "FieldList": 5596, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5596, "order": 0, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "ZSTDnit_frameHeader"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5596, "order": 1, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "ZSTDnit_blockHeader"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5596, "order": 2, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 2, "Name": "ZSTDnit_block"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5596, "order": 3, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 3, "Name": "ZSTDnit_lastBlock"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5596, "order": 4, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 4, "Name": "ZSTDnit_checksum"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5596, "order": 5, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 5, "Name": "ZSTDnit_skippableFrame"}}, "type_references": []}
// [PDB UDT 源码行] {"ipi_index": 102316, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/decompress/../common/../zstd.h", "line": 2527, "module": null, "source_string_id": 102208}
// === 完整 TPI 结构关系引用记录；原文不转换成猜测的 Rust ABI ===
// [TPI 0x101c] {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 3, "Modifiers": ["None", "Const"]}}
// [TPI 0x101d] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4124, "Attrs": 65548}}
// [TPI 0x1188] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [1539, 35]}}
// [TPI 0x1239] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ZSTD_customMem", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x124e] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ZSTD_inBuffer_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1257] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 1539, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 2, "ArgumentList": 4488}}
// [TPI 0x1258] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4695, "Attrs": 65548}}
// [TPI 0x1259] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [1539, 1539]}}
// [TPI 0x125a] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 3, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 2, "ArgumentList": 4697}}
// [TPI 0x125b] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4698, "Attrs": 65548}}
// [TPI 0x125c] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4696, "FieldOffset": 0, "Name": "customAlloc"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4699, "FieldOffset": 8, "Name": "customFree"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 16, "Name": "opaque"}}]}
// [TPI 0x125d] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None"], "FieldList": 4700, "Name": "ZSTD_customMem", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 24}}
// [TPI 0x125e] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "ZSTD_f_zstd1"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "ZSTD_f_zstd1_magicless"}}]}
// [TPI 0x125f] {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 2, "Options": ["None"], "FieldList": 4702, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [TPI 0x1291] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4125, "FieldOffset": 0, "Name": "src"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 8, "Name": "size"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 16, "Name": "pos"}}]}
// [TPI 0x1292] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None"], "FieldList": 4753, "Name": "ZSTD_inBuffer_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 24}}
// [TPI 0x1293] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "ZSTD_dct_auto"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "ZSTD_dct_rawContent"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 2, "Name": "ZSTD_dct_fullDict"}}]}
// [TPI 0x1294] {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 3, "Options": ["None"], "FieldList": 4755, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [TPI 0x135c] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "ZSTD_reset_session_only"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 2, "Name": "ZSTD_reset_parameters"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 3, "Name": "ZSTD_reset_session_and_parameters"}}]}
// [TPI 0x135d] {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 3, "Options": ["None"], "FieldList": 4956, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [TPI 0x13bc] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "ZSTD_dlm_byCopy"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "ZSTD_dlm_byRef"}}]}
// [TPI 0x13bd] {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 2, "Options": ["None"], "FieldList": 5052, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [TPI 0x13db] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ZSTD_outBuffer_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x13e2] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 0, "Name": "dst"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 8, "Name": "size"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 16, "Name": "pos"}}]}
// [TPI 0x13e3] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None"], "FieldList": 5090, "Name": "ZSTD_outBuffer_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 24}}
// [TPI 0x1477] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ZSTD_bounds", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1479] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 0, "Name": "error"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 116, "FieldOffset": 8, "Name": "lowerBound"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 116, "FieldOffset": 12, "Name": "upperBound"}}]}
// [TPI 0x147a] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None"], "FieldList": 5241, "Name": "ZSTD_bounds", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 16}}
// [TPI 0x1544] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ZSTD_frameHeader", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1549] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "ZSTD_d_validateChecksum"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "ZSTD_d_ignoreChecksum"}}]}
// [TPI 0x154a] {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 2, "Options": ["None"], "FieldList": 5449, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [TPI 0x1553] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "ZSTD_rmd_refSingleDDict"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "ZSTD_rmd_refMultipleDDicts"}}]}
// [TPI 0x1554] {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 2, "Options": ["None"], "FieldList": 5459, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [TPI 0x1565] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "ZSTD_frame"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "ZSTD_skippableFrame"}}]}
// [TPI 0x1566] {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 2, "Options": ["None"], "FieldList": 5477, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [TPI 0x1567] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 0, "Name": "frameContentSize"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 8, "Name": "windowSize"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 16, "Name": "blockSizeMax"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5478, "FieldOffset": 20, "Name": "frameType"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 24, "Name": "headerSize"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 28, "Name": "dictID"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 32, "Name": "checksumFlag"}}]}
// [TPI 0x1568] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 7, "Options": ["None"], "FieldList": 5479, "Name": "ZSTD_frameHeader", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 40}}
// [TPI 0x15a5] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 100, "Name": "ZSTD_d_windowLogMax"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1000, "Name": "ZSTD_d_experimentalParam1"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1001, "Name": "ZSTD_d_experimentalParam2"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1002, "Name": "ZSTD_d_experimentalParam3"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1003, "Name": "ZSTD_d_experimentalParam4"}}]}
// [TPI 0x15a6] {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 5, "Options": ["None"], "FieldList": 5541, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [TPI 0x15dc] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "ZSTDnit_frameHeader"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "ZSTDnit_blockHeader"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 2, "Name": "ZSTDnit_block"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 3, "Name": "ZSTDnit_lastBlock"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 4, "Name": "ZSTDnit_checksum"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 5, "Name": "ZSTDnit_skippableFrame"}}]}
// [TPI 0x15dd] {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 6, "Options": ["None"], "FieldList": 5596, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: zstd.h.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===
