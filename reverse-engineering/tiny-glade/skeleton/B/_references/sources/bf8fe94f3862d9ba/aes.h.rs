// Generated from PDB metadata by skeleton/_tools/fill_type_structures.py.
// Version: B
// 类型元数据；原作者源码文本/定义文件见逐项映射；不生成依赖实现。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: aes.h.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB forward_declaration_only] aes_key_st; TPI 0x22a5
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "aes_key_st", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [8875]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] aes_key_st; TPI 0x22ab
// 原始定义文件映射: PDB_UDT_source_line
// size = 244; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 8874, "Name": "aes_key_st", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 244}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8874, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8873, "FieldOffset": 0, "Name": "rd_key"}}, "type_references": [{"index": 8873, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 117, "IndexType": 35, "Size": 240, "Name": ""}}, "nested_types": {"ElementType": {"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8874, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 240, "Name": "rounds"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 129772, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ring-0.16.20/include/GFp/aes.h", "line": 62, "module": null, "source_string_id": 129771}
// === 完整 TPI 结构关系引用记录；原文不转换成猜测的 Rust ABI ===
// [TPI 0x22a5] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "aes_key_st", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x22a9] {"Kind": "LF_ARRAY", "Array": {"ElementType": 117, "IndexType": 35, "Size": 240, "Name": ""}}
// [TPI 0x22aa] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8873, "FieldOffset": 0, "Name": "rd_key"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 240, "Name": "rounds"}}]}
// [TPI 0x22ab] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 8874, "Name": "aes_key_st", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 244}}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: aes.h.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===
