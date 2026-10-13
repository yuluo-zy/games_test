// Generated from PDB metadata by skeleton/_tools/fill_type_structures.py.
// Version: A
// 类型元数据；原作者源码文本/定义文件见逐项映射；不生成依赖实现。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: zstd.h.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB forward_declaration_only] ZSTD_customMem; TPI 0x181a
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ZSTD_customMem", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [6203]
// ------------------------------------------------------------------
// [PDB forward_declaration_only] ZSTD_inBuffer_s; TPI 0x182f
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ZSTD_inBuffer_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [6256]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] ZSTD_customMem; TPI 0x183b
// 原始定义文件映射: PDB_UDT_source_line
// size = 24; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None"], "FieldList": 6202, "Name": "ZSTD_customMem", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 24}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6202, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6201, "FieldOffset": 0, "Name": "customAlloc"}}, "type_references": [{"index": 6201, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 6200, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 6200, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 1539, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 2, "ArgumentList": 4147}}, "nested_types": {}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6202, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4601, "FieldOffset": 8, "Name": "customFree"}}, "type_references": [{"index": 4601, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4600, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4600, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 3, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 2, "ArgumentList": 4599}}, "nested_types": {}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6202, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 16, "Name": "opaque"}}, "type_references": [{"index": 1539, "spelling": "Void / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 99360, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/../common/../zstd.h", "line": 1587, "module": null, "source_string_id": 99359}
// [PDB UDT 源码行] {"ipi_index": 99756, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/decompress/../common/../zstd.h", "line": 1587, "module": null, "source_string_id": 99741}
// [PDB UDT 源码行] {"ipi_index": 114479, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/common/../zstd.h", "line": 1587, "module": null, "source_string_id": 114478}
// [PDB UDT 源码行] {"ipi_index": 114535, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/dictBuilder/../common/../zstd.h", "line": 1587, "module": null, "source_string_id": 114513}
// [PDB UDT 源码行] {"ipi_index": 114711, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/../zstd.h", "line": 1587, "module": null, "source_string_id": 114703}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] <unnamed-tag>; TPI 0x183d
// 原始定义文件映射: PDB_UDT_source_line
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 2, "Options": ["None"], "FieldList": 6204, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6204, "order": 0, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "ZSTD_f_zstd1"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6204, "order": 1, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "ZSTD_f_zstd1_magicless"}}, "type_references": []}
// [PDB UDT 源码行] {"ipi_index": 99361, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/../common/../zstd.h", "line": 1247, "module": null, "source_string_id": 99359}
// [PDB UDT 源码行] {"ipi_index": 99742, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/decompress/../common/../zstd.h", "line": 1247, "module": null, "source_string_id": 99741}
// [PDB UDT 源码行] {"ipi_index": 114523, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/dictBuilder/../common/../zstd.h", "line": 1247, "module": null, "source_string_id": 114513}
// [PDB UDT 源码行] {"ipi_index": 114704, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/../zstd.h", "line": 1247, "module": null, "source_string_id": 114703}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] ZSTD_compressionParameters; TPI 0x183e
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ZSTD_compressionParameters", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [6273]
// ------------------------------------------------------------------
// [PDB forward_declaration_only] ZSTD_frameParameters; TPI 0x183f
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ZSTD_frameParameters", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [6275]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] <unnamed-tag>; TPI 0x1841
// 原始定义文件映射: PDB_UDT_source_line
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 4, "Options": ["None"], "FieldList": 6208, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6208, "order": 0, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "ZSTD_dictDefaultAttach"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6208, "order": 1, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "ZSTD_dictForceAttach"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6208, "order": 2, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 2, "Name": "ZSTD_dictForceCopy"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6208, "order": 3, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 3, "Name": "ZSTD_dictForceLoad"}}, "type_references": []}
// [PDB UDT 源码行] {"ipi_index": 99362, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/../common/../zstd.h", "line": 1266, "module": null, "source_string_id": 99359}
// [PDB UDT 源码行] {"ipi_index": 114524, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/dictBuilder/../common/../zstd.h", "line": 1266, "module": null, "source_string_id": 114513}
// [PDB UDT 源码行] {"ipi_index": 114705, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/../zstd.h", "line": 1266, "module": null, "source_string_id": 114703}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] <unnamed-tag>; TPI 0x1843
// 原始定义文件映射: PDB_UDT_source_line
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 3, "Options": ["None"], "FieldList": 6210, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6210, "order": 0, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "ZSTD_ps_auto"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6210, "order": 1, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "ZSTD_ps_enable"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6210, "order": 2, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 2, "Name": "ZSTD_ps_disable"}}, "type_references": []}
// [PDB UDT 源码行] {"ipi_index": 99363, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/../common/../zstd.h", "line": 1314, "module": null, "source_string_id": 99359}
// [PDB UDT 源码行] {"ipi_index": 114525, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/dictBuilder/../common/../zstd.h", "line": 1314, "module": null, "source_string_id": 114513}
// [PDB UDT 源码行] {"ipi_index": 114706, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/../zstd.h", "line": 1314, "module": null, "source_string_id": 114703}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] <unnamed-tag>; TPI 0x1848
// 原始定义文件映射: PDB_UDT_source_line
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 2, "Options": ["None"], "FieldList": 6215, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6215, "order": 0, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "ZSTD_sf_noBlockDelimiters"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6215, "order": 1, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "ZSTD_sf_explicitBlockDelimiters"}}, "type_references": []}
// [PDB UDT 源码行] {"ipi_index": 99366, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/../common/../zstd.h", "line": 1372, "module": null, "source_string_id": 99359}
// [PDB UDT 源码行] {"ipi_index": 114528, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/dictBuilder/../common/../zstd.h", "line": 1372, "module": null, "source_string_id": 114513}
// [PDB UDT 源码行] {"ipi_index": 114707, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/../zstd.h", "line": 1372, "module": null, "source_string_id": 114703}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] ZSTD_Sequence; TPI 0x1854
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ZSTD_Sequence", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [6279]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] ZSTD_inBuffer_s; TPI 0x1870
// 原始定义文件映射: PDB_UDT_source_line
// size = 24; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None"], "FieldList": 6255, "Name": "ZSTD_inBuffer_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 24}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6255, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4353, "FieldOffset": 0, "Name": "src"}}, "type_references": [{"index": 4353, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4352, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4352, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 3, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 3, "spelling": "Void", "status": "CodeView_simple_type; not Rust source spelling"}}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6255, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 8, "Name": "size"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6255, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 16, "Name": "pos"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 99381, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/../common/../zstd.h", "line": 596, "module": null, "source_string_id": 99359}
// [PDB UDT 源码行] {"ipi_index": 99877, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/decompress/../common/../zstd.h", "line": 596, "module": null, "source_string_id": 99741}
// [PDB UDT 源码行] {"ipi_index": 114543, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/dictBuilder/../common/../zstd.h", "line": 596, "module": null, "source_string_id": 114513}
// [PDB UDT 源码行] {"ipi_index": 114723, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/../zstd.h", "line": 596, "module": null, "source_string_id": 114703}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] <unnamed-tag>; TPI 0x1872
// 原始定义文件映射: PDB_UDT_source_line
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 3, "Options": ["None"], "FieldList": 6257, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6257, "order": 0, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "ZSTD_dct_auto"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6257, "order": 1, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "ZSTD_dct_rawContent"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6257, "order": 2, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 2, "Name": "ZSTD_dct_fullDict"}}, "type_references": []}
// [PDB UDT 源码行] {"ipi_index": 99382, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/../common/../zstd.h", "line": 1236, "module": null, "source_string_id": 99359}
// [PDB UDT 源码行] {"ipi_index": 99795, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/decompress/../common/../zstd.h", "line": 1236, "module": null, "source_string_id": 99741}
// [PDB UDT 源码行] {"ipi_index": 114544, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/dictBuilder/../common/../zstd.h", "line": 1236, "module": null, "source_string_id": 114513}
// [PDB UDT 源码行] {"ipi_index": 114724, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/../zstd.h", "line": 1236, "module": null, "source_string_id": 114703}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] <unnamed-tag>; TPI 0x187f
// 原始定义文件映射: PDB_UDT_source_line
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 9, "Options": ["None"], "FieldList": 6270, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6270, "order": 0, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "ZSTD_fast"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6270, "order": 1, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 2, "Name": "ZSTD_dfast"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6270, "order": 2, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 3, "Name": "ZSTD_greedy"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6270, "order": 3, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 4, "Name": "ZSTD_lazy"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6270, "order": 4, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 5, "Name": "ZSTD_lazy2"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6270, "order": 5, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 6, "Name": "ZSTD_btlazy2"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6270, "order": 6, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 7, "Name": "ZSTD_btopt"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6270, "order": 7, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 8, "Name": "ZSTD_btultra"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6270, "order": 8, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 9, "Name": "ZSTD_btultra2"}}, "type_references": []}
// [PDB UDT 源码行] {"ipi_index": 99388, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/../common/../zstd.h", "line": 258, "module": null, "source_string_id": 99359}
// [PDB UDT 源码行] {"ipi_index": 114520, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/dictBuilder/../common/../zstd.h", "line": 258, "module": null, "source_string_id": 114513}
// [PDB UDT 源码行] {"ipi_index": 114708, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/../zstd.h", "line": 258, "module": null, "source_string_id": 114703}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] ZSTD_compressionParameters; TPI 0x1881
// 原始定义文件映射: PDB_UDT_source_line
// size = 28; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 7, "Options": ["None"], "FieldList": 6272, "Name": "ZSTD_compressionParameters", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 28}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6272, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 0, "Name": "windowLog"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6272, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 4, "Name": "chainLog"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6272, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 8, "Name": "hashLog"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6272, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 12, "Name": "searchLog"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6272, "order": 4, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 16, "Name": "minMatch"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6272, "order": 5, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 20, "Name": "targetLength"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6272, "order": 6, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6271, "FieldOffset": 24, "Name": "strategy"}}, "type_references": [{"index": 6271, "kind": "LF_ENUM", "spelling": "<unnamed-tag>", "status": "PDB_record_name"}]}
// [PDB UDT 源码行] {"ipi_index": 99389, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/../common/../zstd.h", "line": 1215, "module": null, "source_string_id": 99359}
// [PDB UDT 源码行] {"ipi_index": 114521, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/dictBuilder/../common/../zstd.h", "line": 1215, "module": null, "source_string_id": 114513}
// [PDB UDT 源码行] {"ipi_index": 114709, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/../zstd.h", "line": 1215, "module": null, "source_string_id": 114703}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] ZSTD_frameParameters; TPI 0x1883
// 原始定义文件映射: PDB_UDT_source_line
// size = 12; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None"], "FieldList": 6274, "Name": "ZSTD_frameParameters", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 12}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6274, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 116, "FieldOffset": 0, "Name": "contentSizeFlag"}}, "type_references": [{"index": 116, "spelling": "Int32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6274, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 116, "FieldOffset": 4, "Name": "checksumFlag"}}, "type_references": [{"index": 116, "spelling": "Int32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6274, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 116, "FieldOffset": 8, "Name": "noDictIDFlag"}}, "type_references": [{"index": 116, "spelling": "Int32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 99390, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/../common/../zstd.h", "line": 1225, "module": null, "source_string_id": 99359}
// [PDB UDT 源码行] {"ipi_index": 114522, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/dictBuilder/../common/../zstd.h", "line": 1225, "module": null, "source_string_id": 114513}
// [PDB UDT 源码行] {"ipi_index": 114710, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/../zstd.h", "line": 1225, "module": null, "source_string_id": 114703}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] ZSTD_Sequence; TPI 0x1887
// 原始定义文件映射: PDB_UDT_source_line
// size = 16; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 6278, "Name": "ZSTD_Sequence", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 16}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6278, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 0, "Name": "offset"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6278, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 4, "Name": "litLength"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6278, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 8, "Name": "matchLength"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6278, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 12, "Name": "rep"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 99392, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/../common/../zstd.h", "line": 1178, "module": null, "source_string_id": 99359}
// [PDB UDT 源码行] {"ipi_index": 114549, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/dictBuilder/../common/../zstd.h", "line": 1178, "module": null, "source_string_id": 114513}
// [PDB UDT 源码行] {"ipi_index": 114725, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/../zstd.h", "line": 1178, "module": null, "source_string_id": 114703}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] <unnamed-tag>; TPI 0x18cd
// 原始定义文件映射: PDB_UDT_source_line
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 3, "Options": ["None"], "FieldList": 6348, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6348, "order": 0, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "ZSTD_reset_session_only"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6348, "order": 1, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 2, "Name": "ZSTD_reset_parameters"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6348, "order": 2, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 3, "Name": "ZSTD_reset_session_and_parameters"}}, "type_references": []}
// [PDB UDT 源码行] {"ipi_index": 99420, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/../common/../zstd.h", "line": 490, "module": null, "source_string_id": 99359}
// [PDB UDT 源码行] {"ipi_index": 99798, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/decompress/../common/../zstd.h", "line": 490, "module": null, "source_string_id": 99741}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] <unnamed-tag>; TPI 0x18f6
// 原始定义文件映射: PDB_UDT_source_line
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 34, "Options": ["None"], "FieldList": 6389, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6389, "order": 0, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 100, "Name": "ZSTD_c_compressionLevel"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6389, "order": 1, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 101, "Name": "ZSTD_c_windowLog"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6389, "order": 2, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 102, "Name": "ZSTD_c_hashLog"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6389, "order": 3, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 103, "Name": "ZSTD_c_chainLog"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6389, "order": 4, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 104, "Name": "ZSTD_c_searchLog"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6389, "order": 5, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 105, "Name": "ZSTD_c_minMatch"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6389, "order": 6, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 106, "Name": "ZSTD_c_targetLength"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6389, "order": 7, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 107, "Name": "ZSTD_c_strategy"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6389, "order": 8, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 160, "Name": "ZSTD_c_enableLongDistanceMatching"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6389, "order": 9, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 161, "Name": "ZSTD_c_ldmHashLog"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6389, "order": 10, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 162, "Name": "ZSTD_c_ldmMinMatch"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6389, "order": 11, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 163, "Name": "ZSTD_c_ldmBucketSizeLog"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6389, "order": 12, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 164, "Name": "ZSTD_c_ldmHashRateLog"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6389, "order": 13, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 200, "Name": "ZSTD_c_contentSizeFlag"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6389, "order": 14, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 201, "Name": "ZSTD_c_checksumFlag"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6389, "order": 15, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 202, "Name": "ZSTD_c_dictIDFlag"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6389, "order": 16, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 400, "Name": "ZSTD_c_nbWorkers"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6389, "order": 17, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 401, "Name": "ZSTD_c_jobSize"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6389, "order": 18, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 402, "Name": "ZSTD_c_overlapLog"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6389, "order": 19, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 500, "Name": "ZSTD_c_experimentalParam1"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6389, "order": 20, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 10, "Name": "ZSTD_c_experimentalParam2"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6389, "order": 21, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1000, "Name": "ZSTD_c_experimentalParam3"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6389, "order": 22, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1001, "Name": "ZSTD_c_experimentalParam4"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6389, "order": 23, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1002, "Name": "ZSTD_c_experimentalParam5"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6389, "order": 24, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1003, "Name": "ZSTD_c_experimentalParam6"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6389, "order": 25, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1004, "Name": "ZSTD_c_experimentalParam7"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6389, "order": 26, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1005, "Name": "ZSTD_c_experimentalParam8"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6389, "order": 27, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1006, "Name": "ZSTD_c_experimentalParam9"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6389, "order": 28, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1007, "Name": "ZSTD_c_experimentalParam10"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6389, "order": 29, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1008, "Name": "ZSTD_c_experimentalParam11"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6389, "order": 30, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1009, "Name": "ZSTD_c_experimentalParam12"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6389, "order": 31, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1010, "Name": "ZSTD_c_experimentalParam13"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6389, "order": 32, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1011, "Name": "ZSTD_c_experimentalParam14"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6389, "order": 33, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1012, "Name": "ZSTD_c_experimentalParam15"}}, "type_references": []}
// [PDB UDT 源码行] {"ipi_index": 99439, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/../common/../zstd.h", "line": 271, "module": null, "source_string_id": 99359}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] <unnamed-tag>; TPI 0x1909
// 原始定义文件映射: PDB_UDT_source_line
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 2, "Options": ["None"], "FieldList": 6408, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6408, "order": 0, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "ZSTD_dlm_byCopy"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6408, "order": 1, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "ZSTD_dlm_byRef"}}, "type_references": []}
// [PDB UDT 源码行] {"ipi_index": 99449, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/../common/../zstd.h", "line": 1242, "module": null, "source_string_id": 99359}
// [PDB UDT 源码行] {"ipi_index": 99794, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/decompress/../common/../zstd.h", "line": 1242, "module": null, "source_string_id": 99741}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] <unnamed-tag>; TPI 0x1934
// 原始定义文件映射: PDB_UDT_source_line
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 3, "Options": ["None"], "FieldList": 6451, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6451, "order": 0, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "ZSTD_e_continue"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6451, "order": 1, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "ZSTD_e_flush"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6451, "order": 2, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 2, "Name": "ZSTD_e_end"}}, "type_references": []}
// [PDB UDT 源码行] {"ipi_index": 99478, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/../common/../zstd.h", "line": 678, "module": null, "source_string_id": 99359}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] ZSTD_parameters; TPI 0x1983
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ZSTD_parameters", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [6534, 8159]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] ZSTD_parameters; TPI 0x1986
// 原始定义文件映射: PDB_UDT_source_line
// size = 40; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 6533, "Name": "ZSTD_parameters", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 40}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6533, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6206, "FieldOffset": 0, "Name": "cParams"}}, "type_references": [{"index": 6206, "kind": "LF_STRUCTURE", "spelling": "ZSTD_compressionParameters", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6533, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6207, "FieldOffset": 28, "Name": "fParams"}}, "type_references": [{"index": 6207, "kind": "LF_STRUCTURE", "spelling": "ZSTD_frameParameters", "status": "PDB_record_name"}]}
// [PDB UDT 源码行] {"ipi_index": 99520, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/../common/../zstd.h", "line": 1231, "module": null, "source_string_id": 99359}
// [PDB UDT 源码行] {"ipi_index": 114514, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/dictBuilder/../common/../zstd.h", "line": 1231, "module": null, "source_string_id": 114513}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] ZSTD_outBuffer_s; TPI 0x19b4
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ZSTD_outBuffer_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [6588]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] ZSTD_outBuffer_s; TPI 0x19bc
// 原始定义文件映射: PDB_UDT_source_line
// size = 24; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None"], "FieldList": 6587, "Name": "ZSTD_outBuffer_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 24}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6587, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 0, "Name": "dst"}}, "type_references": [{"index": 1539, "spelling": "Void / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6587, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 8, "Name": "size"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6587, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 16, "Name": "pos"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 99547, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/../common/../zstd.h", "line": 602, "module": null, "source_string_id": 99359}
// [PDB UDT 源码行] {"ipi_index": 99758, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/decompress/../common/../zstd.h", "line": 602, "module": null, "source_string_id": 99741}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] ZSTD_bounds; TPI 0x1a4a
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ZSTD_bounds", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [6733]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] ZSTD_bounds; TPI 0x1a4d
// 原始定义文件映射: PDB_UDT_source_line
// size = 16; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None"], "FieldList": 6732, "Name": "ZSTD_bounds", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 16}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6732, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 0, "Name": "error"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6732, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 116, "FieldOffset": 8, "Name": "lowerBound"}}, "type_references": [{"index": 116, "spelling": "Int32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6732, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 116, "FieldOffset": 12, "Name": "upperBound"}}, "type_references": [{"index": 116, "spelling": "Int32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 99627, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/../common/../zstd.h", "line": 445, "module": null, "source_string_id": 99359}
// [PDB UDT 源码行] {"ipi_index": 99871, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/decompress/../common/../zstd.h", "line": 445, "module": null, "source_string_id": 99741}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] ZSTD_frameProgression; TPI 0x1a60
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ZSTD_frameProgression", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [6755]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] ZSTD_frameProgression; TPI 0x1a63
// 原始定义文件映射: PDB_UDT_source_line
// size = 40; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 6, "Options": ["None"], "FieldList": 6754, "Name": "ZSTD_frameProgression", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 40}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6754, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 0, "Name": "ingested"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6754, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 8, "Name": "consumed"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6754, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 16, "Name": "produced"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6754, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 24, "Name": "flushed"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6754, "order": 4, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 32, "Name": "currentJobID"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6754, "order": 5, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 36, "Name": "nbActiveWorkers"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 99645, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/../common/../zstd.h", "line": 2301, "module": null, "source_string_id": 99359}
// === 完整 TPI 结构关系引用记录；原文不转换成猜测的 Rust ABI ===
// [TPI 0x1033] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [1539, 35]}}
// [TPI 0x1100] {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 3, "Modifiers": ["None", "Const"]}}
// [TPI 0x1101] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4352, "Attrs": 65548}}
// [TPI 0x11f7] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [1539, 1539]}}
// [TPI 0x11f8] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 3, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 2, "ArgumentList": 4599}}
// [TPI 0x11f9] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4600, "Attrs": 65548}}
// [TPI 0x181a] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ZSTD_customMem", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x182f] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ZSTD_inBuffer_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1838] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 1539, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 2, "ArgumentList": 4147}}
// [TPI 0x1839] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 6200, "Attrs": 65548}}
// [TPI 0x183a] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6201, "FieldOffset": 0, "Name": "customAlloc"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4601, "FieldOffset": 8, "Name": "customFree"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 16, "Name": "opaque"}}]}
// [TPI 0x183b] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None"], "FieldList": 6202, "Name": "ZSTD_customMem", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 24}}
// [TPI 0x183c] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "ZSTD_f_zstd1"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "ZSTD_f_zstd1_magicless"}}]}
// [TPI 0x183d] {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 2, "Options": ["None"], "FieldList": 6204, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [TPI 0x183e] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ZSTD_compressionParameters", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x183f] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ZSTD_frameParameters", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1840] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "ZSTD_dictDefaultAttach"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "ZSTD_dictForceAttach"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 2, "Name": "ZSTD_dictForceCopy"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 3, "Name": "ZSTD_dictForceLoad"}}]}
// [TPI 0x1841] {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 4, "Options": ["None"], "FieldList": 6208, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [TPI 0x1842] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "ZSTD_ps_auto"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "ZSTD_ps_enable"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 2, "Name": "ZSTD_ps_disable"}}]}
// [TPI 0x1843] {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 3, "Options": ["None"], "FieldList": 6210, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [TPI 0x1847] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "ZSTD_sf_noBlockDelimiters"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "ZSTD_sf_explicitBlockDelimiters"}}]}
// [TPI 0x1848] {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 2, "Options": ["None"], "FieldList": 6215, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [TPI 0x1854] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ZSTD_Sequence", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x186f] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4353, "FieldOffset": 0, "Name": "src"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 8, "Name": "size"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 16, "Name": "pos"}}]}
// [TPI 0x1870] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None"], "FieldList": 6255, "Name": "ZSTD_inBuffer_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 24}}
// [TPI 0x1871] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "ZSTD_dct_auto"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "ZSTD_dct_rawContent"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 2, "Name": "ZSTD_dct_fullDict"}}]}
// [TPI 0x1872] {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 3, "Options": ["None"], "FieldList": 6257, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [TPI 0x187e] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "ZSTD_fast"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 2, "Name": "ZSTD_dfast"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 3, "Name": "ZSTD_greedy"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 4, "Name": "ZSTD_lazy"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 5, "Name": "ZSTD_lazy2"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 6, "Name": "ZSTD_btlazy2"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 7, "Name": "ZSTD_btopt"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 8, "Name": "ZSTD_btultra"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 9, "Name": "ZSTD_btultra2"}}]}
// [TPI 0x187f] {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 9, "Options": ["None"], "FieldList": 6270, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [TPI 0x1880] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 0, "Name": "windowLog"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 4, "Name": "chainLog"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 8, "Name": "hashLog"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 12, "Name": "searchLog"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 16, "Name": "minMatch"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 20, "Name": "targetLength"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6271, "FieldOffset": 24, "Name": "strategy"}}]}
// [TPI 0x1881] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 7, "Options": ["None"], "FieldList": 6272, "Name": "ZSTD_compressionParameters", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 28}}
// [TPI 0x1882] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 116, "FieldOffset": 0, "Name": "contentSizeFlag"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 116, "FieldOffset": 4, "Name": "checksumFlag"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 116, "FieldOffset": 8, "Name": "noDictIDFlag"}}]}
// [TPI 0x1883] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None"], "FieldList": 6274, "Name": "ZSTD_frameParameters", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 12}}
// [TPI 0x1886] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 0, "Name": "offset"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 4, "Name": "litLength"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 8, "Name": "matchLength"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 12, "Name": "rep"}}]}
// [TPI 0x1887] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 6278, "Name": "ZSTD_Sequence", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 16}}
// [TPI 0x18cc] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "ZSTD_reset_session_only"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 2, "Name": "ZSTD_reset_parameters"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 3, "Name": "ZSTD_reset_session_and_parameters"}}]}
// [TPI 0x18cd] {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 3, "Options": ["None"], "FieldList": 6348, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [TPI 0x18f5] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 100, "Name": "ZSTD_c_compressionLevel"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 101, "Name": "ZSTD_c_windowLog"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 102, "Name": "ZSTD_c_hashLog"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 103, "Name": "ZSTD_c_chainLog"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 104, "Name": "ZSTD_c_searchLog"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 105, "Name": "ZSTD_c_minMatch"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 106, "Name": "ZSTD_c_targetLength"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 107, "Name": "ZSTD_c_strategy"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 160, "Name": "ZSTD_c_enableLongDistanceMatching"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 161, "Name": "ZSTD_c_ldmHashLog"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 162, "Name": "ZSTD_c_ldmMinMatch"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 163, "Name": "ZSTD_c_ldmBucketSizeLog"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 164, "Name": "ZSTD_c_ldmHashRateLog"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 200, "Name": "ZSTD_c_contentSizeFlag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 201, "Name": "ZSTD_c_checksumFlag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 202, "Name": "ZSTD_c_dictIDFlag"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 400, "Name": "ZSTD_c_nbWorkers"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 401, "Name": "ZSTD_c_jobSize"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 402, "Name": "ZSTD_c_overlapLog"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 500, "Name": "ZSTD_c_experimentalParam1"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 10, "Name": "ZSTD_c_experimentalParam2"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1000, "Name": "ZSTD_c_experimentalParam3"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1001, "Name": "ZSTD_c_experimentalParam4"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1002, "Name": "ZSTD_c_experimentalParam5"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1003, "Name": "ZSTD_c_experimentalParam6"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1004, "Name": "ZSTD_c_experimentalParam7"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1005, "Name": "ZSTD_c_experimentalParam8"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1006, "Name": "ZSTD_c_experimentalParam9"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1007, "Name": "ZSTD_c_experimentalParam10"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1008, "Name": "ZSTD_c_experimentalParam11"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1009, "Name": "ZSTD_c_experimentalParam12"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1010, "Name": "ZSTD_c_experimentalParam13"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1011, "Name": "ZSTD_c_experimentalParam14"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1012, "Name": "ZSTD_c_experimentalParam15"}}]}
// [TPI 0x18f6] {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 34, "Options": ["None"], "FieldList": 6389, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [TPI 0x1908] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "ZSTD_dlm_byCopy"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "ZSTD_dlm_byRef"}}]}
// [TPI 0x1909] {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 2, "Options": ["None"], "FieldList": 6408, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [TPI 0x1933] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "ZSTD_e_continue"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "ZSTD_e_flush"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 2, "Name": "ZSTD_e_end"}}]}
// [TPI 0x1934] {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 3, "Options": ["None"], "FieldList": 6451, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [TPI 0x1983] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ZSTD_parameters", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1985] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6206, "FieldOffset": 0, "Name": "cParams"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6207, "FieldOffset": 28, "Name": "fParams"}}]}
// [TPI 0x1986] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 6533, "Name": "ZSTD_parameters", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 40}}
// [TPI 0x19b4] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ZSTD_outBuffer_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x19bb] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 0, "Name": "dst"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 8, "Name": "size"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 16, "Name": "pos"}}]}
// [TPI 0x19bc] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None"], "FieldList": 6587, "Name": "ZSTD_outBuffer_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 24}}
// [TPI 0x1a4a] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ZSTD_bounds", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1a4c] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 0, "Name": "error"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 116, "FieldOffset": 8, "Name": "lowerBound"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 116, "FieldOffset": 12, "Name": "upperBound"}}]}
// [TPI 0x1a4d] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None"], "FieldList": 6732, "Name": "ZSTD_bounds", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 16}}
// [TPI 0x1a60] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ZSTD_frameProgression", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1a62] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 0, "Name": "ingested"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 8, "Name": "consumed"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 16, "Name": "produced"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 24, "Name": "flushed"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 32, "Name": "currentJobID"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 36, "Name": "nbActiveWorkers"}}]}
// [TPI 0x1a63] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 6, "Options": ["None"], "FieldList": 6754, "Name": "ZSTD_frameProgression", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 40}}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: zstd.h.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===
