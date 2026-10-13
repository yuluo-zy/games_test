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
// [PDB PDB_definition_record] <unnamed-tag>; TPI 0x1909
// 原始定义文件映射: PDB_UDT_source_line
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 2, "Options": ["None"], "FieldList": 6408, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6408, "order": 0, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "ZSTD_dlm_byCopy"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6408, "order": 1, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "ZSTD_dlm_byRef"}}, "type_references": []}
// [PDB UDT 源码行] {"ipi_index": 99449, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/../common/../zstd.h", "line": 1242, "module": null, "source_string_id": 99359}
// [PDB UDT 源码行] {"ipi_index": 99794, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/decompress/../common/../zstd.h", "line": 1242, "module": null, "source_string_id": 99741}
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
// [PDB forward_declaration_only] ZSTD_frameHeader; TPI 0x1b15
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ZSTD_frameHeader", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [6969]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] <unnamed-tag>; TPI 0x1b1b
// 原始定义文件映射: PDB_UDT_source_line
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 2, "Options": ["None"], "FieldList": 6938, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6938, "order": 0, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "ZSTD_d_validateChecksum"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6938, "order": 1, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "ZSTD_d_ignoreChecksum"}}, "type_references": []}
// [PDB UDT 源码行] {"ipi_index": 99743, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/decompress/../common/../zstd.h", "line": 1254, "module": null, "source_string_id": 99741}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] <unnamed-tag>; TPI 0x1b25
// 原始定义文件映射: PDB_UDT_source_line
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 2, "Options": ["None"], "FieldList": 6948, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6948, "order": 0, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "ZSTD_rmd_refSingleDDict"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6948, "order": 1, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "ZSTD_rmd_refMultipleDDicts"}}, "type_references": []}
// [PDB UDT 源码行] {"ipi_index": 99745, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/decompress/../common/../zstd.h", "line": 1260, "module": null, "source_string_id": 99741}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] <unnamed-tag>; TPI 0x1b37
// 原始定义文件映射: PDB_UDT_source_line
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 2, "Options": ["None"], "FieldList": 6966, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6966, "order": 0, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "ZSTD_frame"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6966, "order": 1, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "ZSTD_skippableFrame"}}, "type_references": []}
// [PDB UDT 源码行] {"ipi_index": 99752, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/decompress/../common/../zstd.h", "line": 2495, "module": null, "source_string_id": 99741}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] ZSTD_frameHeader; TPI 0x1b39
// 原始定义文件映射: PDB_UDT_source_line
// size = 40; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 7, "Options": ["None"], "FieldList": 6968, "Name": "ZSTD_frameHeader", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 40}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6968, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 0, "Name": "frameContentSize"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6968, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 8, "Name": "windowSize"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6968, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 16, "Name": "blockSizeMax"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6968, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6967, "FieldOffset": 20, "Name": "frameType"}}, "type_references": [{"index": 6967, "kind": "LF_ENUM", "spelling": "<unnamed-tag>", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6968, "order": 4, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 24, "Name": "headerSize"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6968, "order": 5, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 28, "Name": "dictID"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6968, "order": 6, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 32, "Name": "checksumFlag"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 99753, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/decompress/../common/../zstd.h", "line": 2496, "module": null, "source_string_id": 99741}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] <unnamed-tag>; TPI 0x1b76
// 原始定义文件映射: PDB_UDT_source_line
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 5, "Options": ["None"], "FieldList": 7029, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7029, "order": 0, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 100, "Name": "ZSTD_d_windowLogMax"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7029, "order": 1, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1000, "Name": "ZSTD_d_experimentalParam1"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7029, "order": 2, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1001, "Name": "ZSTD_d_experimentalParam2"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7029, "order": 3, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1002, "Name": "ZSTD_d_experimentalParam3"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7029, "order": 4, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1003, "Name": "ZSTD_d_experimentalParam4"}}, "type_references": []}
// [PDB UDT 源码行] {"ipi_index": 99810, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/decompress/../common/../zstd.h", "line": 539, "module": null, "source_string_id": 99741}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] <unnamed-tag>; TPI 0x1bae
// 原始定义文件映射: PDB_UDT_source_line
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 6, "Options": ["None"], "FieldList": 7085, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7085, "order": 0, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "ZSTDnit_frameHeader"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7085, "order": 1, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "ZSTDnit_blockHeader"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7085, "order": 2, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 2, "Name": "ZSTDnit_block"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7085, "order": 3, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 3, "Name": "ZSTDnit_lastBlock"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7085, "order": 4, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 4, "Name": "ZSTDnit_checksum"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7085, "order": 5, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 5, "Name": "ZSTDnit_skippableFrame"}}, "type_references": []}
// [PDB UDT 源码行] {"ipi_index": 99851, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/decompress/../common/../zstd.h", "line": 2527, "module": null, "source_string_id": 99741}
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
// [TPI 0x186f] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4353, "FieldOffset": 0, "Name": "src"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 8, "Name": "size"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 16, "Name": "pos"}}]}
// [TPI 0x1870] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None"], "FieldList": 6255, "Name": "ZSTD_inBuffer_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 24}}
// [TPI 0x1871] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "ZSTD_dct_auto"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "ZSTD_dct_rawContent"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 2, "Name": "ZSTD_dct_fullDict"}}]}
// [TPI 0x1872] {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 3, "Options": ["None"], "FieldList": 6257, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [TPI 0x18cc] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "ZSTD_reset_session_only"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 2, "Name": "ZSTD_reset_parameters"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 3, "Name": "ZSTD_reset_session_and_parameters"}}]}
// [TPI 0x18cd] {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 3, "Options": ["None"], "FieldList": 6348, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [TPI 0x1908] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "ZSTD_dlm_byCopy"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "ZSTD_dlm_byRef"}}]}
// [TPI 0x1909] {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 2, "Options": ["None"], "FieldList": 6408, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [TPI 0x19b4] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ZSTD_outBuffer_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x19bb] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 0, "Name": "dst"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 8, "Name": "size"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 16, "Name": "pos"}}]}
// [TPI 0x19bc] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None"], "FieldList": 6587, "Name": "ZSTD_outBuffer_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 24}}
// [TPI 0x1a4a] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ZSTD_bounds", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1a4c] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 0, "Name": "error"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 116, "FieldOffset": 8, "Name": "lowerBound"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 116, "FieldOffset": 12, "Name": "upperBound"}}]}
// [TPI 0x1a4d] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None"], "FieldList": 6732, "Name": "ZSTD_bounds", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 16}}
// [TPI 0x1b15] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ZSTD_frameHeader", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1b1a] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "ZSTD_d_validateChecksum"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "ZSTD_d_ignoreChecksum"}}]}
// [TPI 0x1b1b] {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 2, "Options": ["None"], "FieldList": 6938, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [TPI 0x1b24] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "ZSTD_rmd_refSingleDDict"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "ZSTD_rmd_refMultipleDDicts"}}]}
// [TPI 0x1b25] {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 2, "Options": ["None"], "FieldList": 6948, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [TPI 0x1b36] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "ZSTD_frame"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "ZSTD_skippableFrame"}}]}
// [TPI 0x1b37] {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 2, "Options": ["None"], "FieldList": 6966, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [TPI 0x1b38] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 0, "Name": "frameContentSize"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 8, "Name": "windowSize"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 16, "Name": "blockSizeMax"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6967, "FieldOffset": 20, "Name": "frameType"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 24, "Name": "headerSize"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 28, "Name": "dictID"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 32, "Name": "checksumFlag"}}]}
// [TPI 0x1b39] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 7, "Options": ["None"], "FieldList": 6968, "Name": "ZSTD_frameHeader", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 40}}
// [TPI 0x1b75] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 100, "Name": "ZSTD_d_windowLogMax"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1000, "Name": "ZSTD_d_experimentalParam1"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1001, "Name": "ZSTD_d_experimentalParam2"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1002, "Name": "ZSTD_d_experimentalParam3"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1003, "Name": "ZSTD_d_experimentalParam4"}}]}
// [TPI 0x1b76] {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 5, "Options": ["None"], "FieldList": 7029, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [TPI 0x1bad] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "ZSTDnit_frameHeader"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "ZSTDnit_blockHeader"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 2, "Name": "ZSTDnit_block"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 3, "Name": "ZSTDnit_lastBlock"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 4, "Name": "ZSTDnit_checksum"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 5, "Name": "ZSTDnit_skippableFrame"}}]}
// [TPI 0x1bae] {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 6, "Options": ["None"], "FieldList": 7085, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: zstd.h.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===
