// Generated from PDB metadata by skeleton/_tools/fill_type_structures.py.
// Version: A
// 类型元数据；原作者源码文本/定义文件见逐项映射；不生成依赖实现。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: cmsgmt.c.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB forward_declaration_only] GAMUTCHAIN; TPI 0x17d1
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "GAMUTCHAIN", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [6099]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] GAMUTCHAIN; TPI 0x17d3
// 原始定义文件映射: PDB_UDT_source_line
// size = 32; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 6098, "Name": "GAMUTCHAIN", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 32}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6098, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 0, "Name": "hInput"}}, "type_references": [{"index": 1539, "spelling": "Void / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6098, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "hForward"}}, "type_references": [{"index": 1539, "spelling": "Void / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6098, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 16, "Name": "hReverse"}}, "type_references": [{"index": 1539, "spelling": "Void / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6098, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 65, "FieldOffset": 24, "Name": "Threshold"}}, "type_references": [{"index": 65, "spelling": "Float64", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 94771, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lcms2-sys-4.0.1/vendor/src/cmsgmt.c", "line": 198, "module": null, "source_string_id": 94770}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] cmsTACestimator; TPI 0x17d5
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsTACestimator", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [6103]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] cmsTACestimator; TPI 0x17d7
// 原始定义文件映射: PDB_UDT_source_line
// size = 88; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 6102, "Name": "cmsTACestimator", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 88}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6102, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 0, "Name": "nOutputChans"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6102, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "hRoundTrip"}}, "type_references": [{"index": 1539, "spelling": "Void / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6102, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 64, "FieldOffset": 16, "Name": "MaxTAC"}}, "type_references": [{"index": 64, "spelling": "Float32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6102, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5265, "FieldOffset": 20, "Name": "MaxInput"}}, "type_references": [{"index": 5265, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 64, "IndexType": 35, "Size": 64, "Name": ""}}, "nested_types": {"ElementType": {"index": 64, "spelling": "Float32", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [PDB UDT 源码行] {"ipi_index": 94774, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lcms2-sys-4.0.1/vendor/src/cmsgmt.c", "line": 415, "module": null, "source_string_id": 94770}
// === 完整 TPI 结构关系引用记录；原文不转换成猜测的 Rust ABI ===
// [TPI 0x1491] {"Kind": "LF_ARRAY", "Array": {"ElementType": 64, "IndexType": 35, "Size": 64, "Name": ""}}
// [TPI 0x17d1] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "GAMUTCHAIN", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x17d2] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 0, "Name": "hInput"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "hForward"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 16, "Name": "hReverse"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 65, "FieldOffset": 24, "Name": "Threshold"}}]}
// [TPI 0x17d3] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 6098, "Name": "GAMUTCHAIN", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 32}}
// [TPI 0x17d5] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsTACestimator", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x17d6] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 0, "Name": "nOutputChans"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "hRoundTrip"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 64, "FieldOffset": 16, "Name": "MaxTAC"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5265, "FieldOffset": 20, "Name": "MaxInput"}}]}
// [TPI 0x17d7] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 6102, "Name": "cmsTACestimator", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 88}}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: cmsgmt.c.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===
