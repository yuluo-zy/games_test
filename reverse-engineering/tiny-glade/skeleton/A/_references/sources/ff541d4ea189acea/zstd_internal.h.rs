// Generated from PDB metadata by skeleton/_tools/fill_type_structures.py.
// Version: A
// 类型元数据；原作者源码文本/定义文件见逐项映射；不生成依赖实现。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: zstd_internal.h.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB forward_declaration_only] seqStore_t; TPI 0x1825
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "seqStore_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [6237]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] <unnamed-tag>; TPI 0x1846
// 原始定义文件映射: PDB_UDT_source_line
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 2, "Options": ["None"], "FieldList": 6213, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6213, "order": 0, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "ZSTD_bm_buffered"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6213, "order": 1, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "ZSTD_bm_stable"}}, "type_references": []}
// [PDB UDT 源码行] {"ipi_index": 99365, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/../common/zstd_internal.h", "line": 277, "module": null, "source_string_id": 99364}
// [PDB UDT 源码行] {"ipi_index": 99747, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/decompress/../common/zstd_internal.h", "line": 277, "module": null, "source_string_id": 99737}
// [PDB UDT 源码行] {"ipi_index": 114527, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/dictBuilder/../common/zstd_internal.h", "line": 277, "module": null, "source_string_id": 114526}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] seqDef_s; TPI 0x1858
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "seqDef_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [6281]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] <unnamed-tag>; TPI 0x185b
// 原始定义文件映射: PDB_UDT_source_line
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 3, "Options": ["None"], "FieldList": 6234, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6234, "order": 0, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "ZSTD_llt_none"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6234, "order": 1, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "ZSTD_llt_literalLength"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6234, "order": 2, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 2, "Name": "ZSTD_llt_matchLength"}}, "type_references": []}
// [PDB UDT 源码行] {"ipi_index": 99375, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/../common/zstd_internal.h", "line": 293, "module": null, "source_string_id": 99364}
// [PDB UDT 源码行] {"ipi_index": 114537, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/dictBuilder/../common/zstd_internal.h", "line": 293, "module": null, "source_string_id": 114526}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] seqStore_t; TPI 0x185d
// 原始定义文件映射: PDB_UDT_source_line
// size = 80; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 11, "Options": ["None"], "FieldList": 6236, "Name": "seqStore_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 80}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6236, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6233, "FieldOffset": 0, "Name": "sequencesStart"}}, "type_references": [{"index": 6233, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 6232, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 6232, "kind": "LF_STRUCTURE", "spelling": "seqDef_s", "status": "PDB_record_name"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6236, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6233, "FieldOffset": 8, "Name": "sequences"}}, "type_references": [{"index": 6233, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 6232, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 6232, "kind": "LF_STRUCTURE", "spelling": "seqDef_s", "status": "PDB_record_name"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6236, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1568, "FieldOffset": 16, "Name": "litStart"}}, "type_references": [{"index": 1568, "spelling": "UnsignedCharacter / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6236, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1568, "FieldOffset": 24, "Name": "lit"}}, "type_references": [{"index": 1568, "spelling": "UnsignedCharacter / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6236, "order": 4, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1568, "FieldOffset": 32, "Name": "llCode"}}, "type_references": [{"index": 1568, "spelling": "UnsignedCharacter / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6236, "order": 5, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1568, "FieldOffset": 40, "Name": "mlCode"}}, "type_references": [{"index": 1568, "spelling": "UnsignedCharacter / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6236, "order": 6, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1568, "FieldOffset": 48, "Name": "ofCode"}}, "type_references": [{"index": 1568, "spelling": "UnsignedCharacter / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6236, "order": 7, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 56, "Name": "maxNbSeq"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6236, "order": 8, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 64, "Name": "maxNbLit"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6236, "order": 9, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6235, "FieldOffset": 72, "Name": "longLengthType"}}, "type_references": [{"index": 6235, "kind": "LF_ENUM", "spelling": "<unnamed-tag>", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6236, "order": 10, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 76, "Name": "longLengthPos"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 99376, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/../common/zstd_internal.h", "line": 299, "module": null, "source_string_id": 99364}
// [PDB UDT 源码行] {"ipi_index": 114538, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/dictBuilder/../common/zstd_internal.h", "line": 299, "module": null, "source_string_id": 114526}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] seqDef_s; TPI 0x1889
// 原始定义文件映射: PDB_UDT_source_line
// size = 8; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None"], "FieldList": 6280, "Name": "seqDef_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 8}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6280, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 0, "Name": "offBase"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6280, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 33, "FieldOffset": 4, "Name": "litLength"}}, "type_references": [{"index": 33, "spelling": "UInt16Short", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6280, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 33, "FieldOffset": 6, "Name": "mlBase"}}, "type_references": [{"index": 33, "spelling": "UInt16Short", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 99393, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/../common/zstd_internal.h", "line": 286, "module": null, "source_string_id": 99364}
// [PDB UDT 源码行] {"ipi_index": 114550, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/dictBuilder/../common/zstd_internal.h", "line": 286, "module": null, "source_string_id": 114526}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] <unnamed-tag>; TPI 0x18ae
// 原始定义文件映射: PDB_UDT_source_line
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 4, "Options": ["None"], "FieldList": 6317, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6317, "order": 0, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "set_basic"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6317, "order": 1, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "set_rle"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6317, "order": 2, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 2, "Name": "set_compressed"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6317, "order": 3, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 3, "Name": "set_repeat"}}, "type_references": []}
// [PDB UDT 源码行] {"ipi_index": 99403, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/../common/zstd_internal.h", "line": 99, "module": null, "source_string_id": 99364}
// [PDB UDT 源码行] {"ipi_index": 114560, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/dictBuilder/../common/zstd_internal.h", "line": 99, "module": null, "source_string_id": 114526}
// [PDB UDT 源码行] {"ipi_index": 115556, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/decompress/../common/zstd_internal.h", "line": 99, "module": null, "source_string_id": 99737}
// === 完整 TPI 结构关系引用记录；原文不转换成猜测的 Rust ABI ===
// [TPI 0x1825] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "seqStore_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1845] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "ZSTD_bm_buffered"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "ZSTD_bm_stable"}}]}
// [TPI 0x1846] {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 2, "Options": ["None"], "FieldList": 6213, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [TPI 0x1858] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "seqDef_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1859] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 6232, "Attrs": 65548}}
// [TPI 0x185a] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "ZSTD_llt_none"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "ZSTD_llt_literalLength"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 2, "Name": "ZSTD_llt_matchLength"}}]}
// [TPI 0x185b] {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 3, "Options": ["None"], "FieldList": 6234, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [TPI 0x185c] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6233, "FieldOffset": 0, "Name": "sequencesStart"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6233, "FieldOffset": 8, "Name": "sequences"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1568, "FieldOffset": 16, "Name": "litStart"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1568, "FieldOffset": 24, "Name": "lit"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1568, "FieldOffset": 32, "Name": "llCode"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1568, "FieldOffset": 40, "Name": "mlCode"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1568, "FieldOffset": 48, "Name": "ofCode"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 56, "Name": "maxNbSeq"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 64, "Name": "maxNbLit"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6235, "FieldOffset": 72, "Name": "longLengthType"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 76, "Name": "longLengthPos"}}]}
// [TPI 0x185d] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 11, "Options": ["None"], "FieldList": 6236, "Name": "seqStore_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 80}}
// [TPI 0x1888] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 0, "Name": "offBase"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 33, "FieldOffset": 4, "Name": "litLength"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 33, "FieldOffset": 6, "Name": "mlBase"}}]}
// [TPI 0x1889] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None"], "FieldList": 6280, "Name": "seqDef_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 8}}
// [TPI 0x18ad] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "set_basic"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "set_rle"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 2, "Name": "set_compressed"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 3, "Name": "set_repeat"}}]}
// [TPI 0x18ae] {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 4, "Options": ["None"], "FieldList": 6317, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: zstd_internal.h.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===
