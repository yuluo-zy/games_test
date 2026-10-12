// Generated from PDB metadata by skeleton/_tools/fill_type_structures.py.
// Version: B
// 类型元数据；原作者源码文本/定义文件见逐项映射；不生成依赖实现。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: zstd_v01.c.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB forward_declaration_only] blockProperties_t; TPI 0x1935
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "blockProperties_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [6455, 7983]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] <unnamed-tag>; TPI 0x1f2d
// 原始定义文件映射: PDB_UDT_source_line
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 4, "Options": ["None"], "FieldList": 7980, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7980, "order": 0, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "bt_compressed"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7980, "order": 1, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "bt_raw"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7980, "order": 2, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 2, "Name": "bt_rle"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7980, "order": 3, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 3, "Name": "bt_end"}}, "type_references": []}
// [PDB UDT 源码行] {"ipi_index": 128888, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v04.c", "line": 384, "module": null, "source_string_id": 128887}
// [PDB UDT 源码行] {"ipi_index": 129018, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v06.c", "line": 477, "module": null, "source_string_id": 128992}
// [PDB UDT 源码行] {"ipi_index": 129137, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v07.c", "line": 2715, "module": null, "source_string_id": 129109}
// [PDB UDT 源码行] {"ipi_index": 129285, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v05.c", "line": 497, "module": null, "source_string_id": 129258}
// [PDB UDT 源码行] {"ipi_index": 129426, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v02.c", "line": 2764, "module": null, "source_string_id": 129425}
// [PDB UDT 源码行] {"ipi_index": 129469, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v01.c", "line": 1402, "module": null, "source_string_id": 129468}
// [PDB UDT 源码行] {"ipi_index": 129513, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v03.c", "line": 2405, "module": null, "source_string_id": 129512}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] blockProperties_t; TPI 0x1f2f
// 原始定义文件映射: PDB_UDT_source_line
// size = 8; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 7982, "Name": "blockProperties_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 8}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7982, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7981, "FieldOffset": 0, "Name": "blockType"}}, "type_references": [{"index": 7981, "kind": "LF_ENUM", "spelling": "<unnamed-tag>", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7982, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 4, "Name": "origSize"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 128889, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v04.c", "line": 2466, "module": null, "source_string_id": 128887}
// [PDB UDT 源码行] {"ipi_index": 129042, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v06.c", "line": 3018, "module": null, "source_string_id": 128992}
// [PDB UDT 源码行] {"ipi_index": 129171, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v07.c", "line": 3247, "module": null, "source_string_id": 129109}
// [PDB UDT 源码行] {"ipi_index": 129308, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v05.c", "line": 2611, "module": null, "source_string_id": 129258}
// [PDB UDT 源码行] {"ipi_index": 129427, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v02.c", "line": 2766, "module": null, "source_string_id": 129425}
// [PDB UDT 源码行] {"ipi_index": 129470, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v01.c", "line": 1404, "module": null, "source_string_id": 129468}
// [PDB UDT 源码行] {"ipi_index": 129514, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v03.c", "line": 2407, "module": null, "source_string_id": 129512}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] FSE_DState_t; TPI 0x1f63
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "FSE_DState_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [8040]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] FSE_DState_t; TPI 0x1f68
// 原始定义文件映射: PDB_UDT_source_line
// size = 16; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 8039, "Name": "FSE_DState_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 16}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8039, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 0, "Name": "state"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8039, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4125, "FieldOffset": 8, "Name": "table"}}, "type_references": [{"index": 4125, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4124, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4124, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 3, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 3, "spelling": "Void", "status": "CodeView_simple_type; not Rust source spelling"}}}}}]}
// [PDB UDT 源码行] {"ipi_index": 128923, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v04.c", "line": 848, "module": null, "source_string_id": 128887}
// [PDB UDT 源码行] {"ipi_index": 129435, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v02.c", "line": 672, "module": null, "source_string_id": 129425}
// [PDB UDT 源码行] {"ipi_index": 129482, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v01.c", "line": 85, "module": null, "source_string_id": 129468}
// [PDB UDT 源码行] {"ipi_index": 129517, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v03.c", "line": 674, "module": null, "source_string_id": 129512}
// [PDB UDT 源码行] {"ipi_index": 134914, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/common/fse.h", "line": 437, "module": null, "source_string_id": 134913}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] seq_t; TPI 0x1f70
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "seq_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [8052, 8203]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] seq_t; TPI 0x1f74
// 原始定义文件映射: PDB_UDT_source_line
// size = 24; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None"], "FieldList": 8051, "Name": "seq_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 24}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8051, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 0, "Name": "litLength"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8051, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 8, "Name": "offset"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8051, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 16, "Name": "matchLength"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 128930, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v04.c", "line": 2789, "module": null, "source_string_id": 128887}
// [PDB UDT 源码行] {"ipi_index": 129437, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v02.c", "line": 3022, "module": null, "source_string_id": 129425}
// [PDB UDT 源码行] {"ipi_index": 129492, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v01.c", "line": 1673, "module": null, "source_string_id": 129468}
// [PDB UDT 源码行] {"ipi_index": 129519, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v03.c", "line": 2663, "module": null, "source_string_id": 129512}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] seqState_t; TPI 0x1f78
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "seqState_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [8061, 8206, 8327, 8464, 8545, 8650]
// ------------------------------------------------------------------
// [PDB forward_declaration_only] FSE_DTableHeader; TPI 0x1fad
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "FSE_DTableHeader", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [8112]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] FSE_DTableHeader; TPI 0x1fb0
// 原始定义文件映射: PDB_UDT_source_line
// size = 4; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 8111, "Name": "FSE_DTableHeader", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 4}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8111, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 33, "FieldOffset": 0, "Name": "tableLog"}}, "type_references": [{"index": 33, "spelling": "UInt16Short", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8111, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 33, "FieldOffset": 2, "Name": "fastMode"}}, "type_references": [{"index": 33, "spelling": "UInt16Short", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 128978, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v04.c", "line": 874, "module": null, "source_string_id": 128887}
// [PDB UDT 源码行] {"ipi_index": 129458, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v02.c", "line": 699, "module": null, "source_string_id": 129425}
// [PDB UDT 源码行] {"ipi_index": 129501, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v01.c", "line": 390, "module": null, "source_string_id": 129468}
// [PDB UDT 源码行] {"ipi_index": 129532, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v03.c", "line": 701, "module": null, "source_string_id": 129512}
// [PDB UDT 源码行] {"ipi_index": 134925, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/common/fse.h", "line": 591, "module": null, "source_string_id": 134913}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] FSE_decode_t; TPI 0x1fb1
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "FSE_decode_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [8116]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] FSE_decode_t; TPI 0x1fb4
// 原始定义文件映射: PDB_UDT_source_line
// size = 4; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None"], "FieldList": 8115, "Name": "FSE_decode_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 4}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8115, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 33, "FieldOffset": 0, "Name": "newState"}}, "type_references": [{"index": 33, "spelling": "UInt16Short", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8115, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 32, "FieldOffset": 2, "Name": "symbol"}}, "type_references": [{"index": 32, "spelling": "UnsignedCharacter", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8115, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 32, "FieldOffset": 3, "Name": "nbBits"}}, "type_references": [{"index": 32, "spelling": "UnsignedCharacter", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 128979, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v04.c", "line": 879, "module": null, "source_string_id": 128887}
// [PDB UDT 源码行] {"ipi_index": 129459, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v02.c", "line": 704, "module": null, "source_string_id": 129425}
// [PDB UDT 源码行] {"ipi_index": 129502, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v01.c", "line": 125, "module": null, "source_string_id": 129468}
// [PDB UDT 源码行] {"ipi_index": 129533, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v03.c", "line": 706, "module": null, "source_string_id": 129512}
// [PDB UDT 源码行] {"ipi_index": 134924, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/common/fse.h", "line": 596, "module": null, "source_string_id": 134913}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] FSE_DStream_t; TPI 0x21b5
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "FSE_DStream_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [8633]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] FSE_DStream_t; TPI 0x21b9
// 原始定义文件映射: PDB_UDT_source_line
// size = 32; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 8033, "Name": "FSE_DStream_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 32}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8033, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 0, "Name": "bitContainer"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8033, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 8, "Name": "bitsConsumed"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8033, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4428, "FieldOffset": 16, "Name": "ptr"}}, "type_references": [{"index": 4428, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4427, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4427, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 112, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 112, "spelling": "NarrowCharacter", "status": "CodeView_simple_type; not Rust source spelling"}}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8033, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4428, "FieldOffset": 24, "Name": "start"}}, "type_references": [{"index": 4428, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4427, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4427, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 112, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 112, "spelling": "NarrowCharacter", "status": "CodeView_simple_type; not Rust source spelling"}}}}}]}
// [PDB UDT 源码行] {"ipi_index": 129476, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v01.c", "line": 77, "module": null, "source_string_id": 129468}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] seqState_t; TPI 0x21ca
// 原始定义文件映射: PDB_UDT_source_line
// size = 104; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 7, "Options": ["None"], "FieldList": 8649, "Name": "seqState_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 104}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8649, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8629, "FieldOffset": 0, "Name": "DStream"}}, "type_references": [{"index": 8629, "kind": "LF_STRUCTURE", "spelling": "FSE_DStream_t", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8649, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8035, "FieldOffset": 32, "Name": "stateLL"}}, "type_references": [{"index": 8035, "kind": "LF_STRUCTURE", "spelling": "FSE_DState_t", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8649, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8035, "FieldOffset": 48, "Name": "stateOffb"}}, "type_references": [{"index": 8035, "kind": "LF_STRUCTURE", "spelling": "FSE_DState_t", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8649, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8035, "FieldOffset": 64, "Name": "stateML"}}, "type_references": [{"index": 8035, "kind": "LF_STRUCTURE", "spelling": "FSE_DState_t", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8649, "order": 4, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 80, "Name": "prevOffset"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8649, "order": 5, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4711, "FieldOffset": 88, "Name": "dumps"}}, "type_references": [{"index": 4711, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4710, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4710, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 32, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 32, "spelling": "UnsignedCharacter", "status": "CodeView_simple_type; not Rust source spelling"}}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8649, "order": 6, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4711, "FieldOffset": 96, "Name": "dumpsEnd"}}, "type_references": [{"index": 4711, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4710, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4710, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 32, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 32, "spelling": "UnsignedCharacter", "status": "CodeView_simple_type; not Rust source spelling"}}}}}]}
// [PDB UDT 源码行] {"ipi_index": 129493, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v01.c", "line": 1679, "module": null, "source_string_id": 129468}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] HUF_DElt; TPI 0x21cb
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "HUF_DElt", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [8656]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] HUF_DElt; TPI 0x21d0
// 原始定义文件映射: PDB_UDT_source_line
// size = 2; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 8077, "Name": "HUF_DElt", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 2}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8077, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 32, "FieldOffset": 0, "Name": "byte"}}, "type_references": [{"index": 32, "spelling": "UnsignedCharacter", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8077, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 32, "FieldOffset": 1, "Name": "nbBits"}}, "type_references": [{"index": 32, "spelling": "UnsignedCharacter", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 129496, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v01.c", "line": 952, "module": null, "source_string_id": 129468}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] ZSTDv01_Dctx_s; TPI 0x21d1
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ZSTDv01_Dctx_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [8662]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] ZSTDv01_Dctx_s; TPI 0x21d6
// 原始定义文件映射: PDB_UDT_source_line
// size = 10288; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 8, "Options": ["None"], "FieldList": 8661, "Name": "ZSTDv01_Dctx_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 10288}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8661, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7988, "FieldOffset": 0, "Name": "LLTable"}}, "type_references": [{"index": 7988, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 117, "IndexType": 35, "Size": 4100, "Name": ""}}, "nested_types": {"ElementType": {"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8661, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7989, "FieldOffset": 4100, "Name": "OffTable"}}, "type_references": [{"index": 7989, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 117, "IndexType": 35, "Size": 2052, "Name": ""}}, "nested_types": {"ElementType": {"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8661, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7988, "FieldOffset": 6152, "Name": "MLTable"}}, "type_references": [{"index": 7988, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 117, "IndexType": 35, "Size": 4100, "Name": ""}}, "nested_types": {"ElementType": {"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8661, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 10256, "Name": "previousDstEnd"}}, "type_references": [{"index": 1539, "spelling": "Void / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8661, "order": 4, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 10264, "Name": "base"}}, "type_references": [{"index": 1539, "spelling": "Void / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8661, "order": 5, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 10272, "Name": "expected"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8661, "order": 6, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7981, "FieldOffset": 10280, "Name": "bType"}}, "type_references": [{"index": 7981, "kind": "LF_ENUM", "spelling": "<unnamed-tag>", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8661, "order": 7, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 10284, "Name": "phase"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 129498, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v01.c", "line": 1833, "module": null, "source_string_id": 129468}
// === 完整 TPI 结构关系引用记录；原文不转换成猜测的 Rust ABI ===
// [TPI 0x101c] {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 3, "Modifiers": ["None", "Const"]}}
// [TPI 0x101d] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4124, "Attrs": 65548}}
// [TPI 0x114b] {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 112, "Modifiers": ["None", "Const"]}}
// [TPI 0x114c] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4427, "Attrs": 65548}}
// [TPI 0x1266] {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 32, "Modifiers": ["None", "Const"]}}
// [TPI 0x1267] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4710, "Attrs": 65548}}
// [TPI 0x1935] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "blockProperties_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1f2c] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "bt_compressed"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "bt_raw"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 2, "Name": "bt_rle"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 3, "Name": "bt_end"}}]}
// [TPI 0x1f2d] {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 4, "Options": ["None"], "FieldList": 7980, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [TPI 0x1f2e] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7981, "FieldOffset": 0, "Name": "blockType"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 4, "Name": "origSize"}}]}
// [TPI 0x1f2f] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 7982, "Name": "blockProperties_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 8}}
// [TPI 0x1f34] {"Kind": "LF_ARRAY", "Array": {"ElementType": 117, "IndexType": 35, "Size": 4100, "Name": ""}}
// [TPI 0x1f35] {"Kind": "LF_ARRAY", "Array": {"ElementType": 117, "IndexType": 35, "Size": 2052, "Name": ""}}
// [TPI 0x1f61] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 0, "Name": "bitContainer"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 8, "Name": "bitsConsumed"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4428, "FieldOffset": 16, "Name": "ptr"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4428, "FieldOffset": 24, "Name": "start"}}]}
// [TPI 0x1f63] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "FSE_DState_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1f67] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 0, "Name": "state"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4125, "FieldOffset": 8, "Name": "table"}}]}
// [TPI 0x1f68] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 8039, "Name": "FSE_DState_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 16}}
// [TPI 0x1f70] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "seq_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1f73] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 0, "Name": "litLength"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 8, "Name": "offset"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 16, "Name": "matchLength"}}]}
// [TPI 0x1f74] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None"], "FieldList": 8051, "Name": "seq_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 24}}
// [TPI 0x1f78] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "seqState_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1f8d] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 32, "FieldOffset": 0, "Name": "byte"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 32, "FieldOffset": 1, "Name": "nbBits"}}]}
// [TPI 0x1fad] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "FSE_DTableHeader", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1faf] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 33, "FieldOffset": 0, "Name": "tableLog"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 33, "FieldOffset": 2, "Name": "fastMode"}}]}
// [TPI 0x1fb0] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 8111, "Name": "FSE_DTableHeader", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 4}}
// [TPI 0x1fb1] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "FSE_decode_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1fb3] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 33, "FieldOffset": 0, "Name": "newState"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 32, "FieldOffset": 2, "Name": "symbol"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 32, "FieldOffset": 3, "Name": "nbBits"}}]}
// [TPI 0x1fb4] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None"], "FieldList": 8115, "Name": "FSE_decode_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 4}}
// [TPI 0x21b5] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "FSE_DStream_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x21b9] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 8033, "Name": "FSE_DStream_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 32}}
// [TPI 0x21c9] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8629, "FieldOffset": 0, "Name": "DStream"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8035, "FieldOffset": 32, "Name": "stateLL"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8035, "FieldOffset": 48, "Name": "stateOffb"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8035, "FieldOffset": 64, "Name": "stateML"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 80, "Name": "prevOffset"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4711, "FieldOffset": 88, "Name": "dumps"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4711, "FieldOffset": 96, "Name": "dumpsEnd"}}]}
// [TPI 0x21ca] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 7, "Options": ["None"], "FieldList": 8649, "Name": "seqState_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 104}}
// [TPI 0x21cb] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "HUF_DElt", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x21d0] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 8077, "Name": "HUF_DElt", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 2}}
// [TPI 0x21d1] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ZSTDv01_Dctx_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x21d5] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7988, "FieldOffset": 0, "Name": "LLTable"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7989, "FieldOffset": 4100, "Name": "OffTable"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7988, "FieldOffset": 6152, "Name": "MLTable"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 10256, "Name": "previousDstEnd"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 10264, "Name": "base"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 10272, "Name": "expected"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7981, "FieldOffset": 10280, "Name": "bType"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 10284, "Name": "phase"}}]}
// [TPI 0x21d6] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 8, "Options": ["None"], "FieldList": 8661, "Name": "ZSTDv01_Dctx_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 10288}}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: zstd_v01.c.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===
