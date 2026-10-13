// Generated from PDB metadata by skeleton/_tools/fill_type_structures.py.
// Version: C
// 类型元数据；原作者源码文本/定义文件见逐项映射；不生成依赖实现。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: winnt.h.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB forward_declaration_only] _RTL_CRITICAL_SECTION; TPI 0x161a
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_RTL_CRITICAL_SECTION", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [5665]
// ------------------------------------------------------------------
// [PDB forward_declaration_only] _RTL_CRITICAL_SECTION_DEBUG; TPI 0x161e
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_RTL_CRITICAL_SECTION_DEBUG", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [5668]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] _RTL_CRITICAL_SECTION; TPI 0x1621
// 原始定义文件映射: PDB_UDT_source_line
// size = 40; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 6, "Options": ["None"], "FieldList": 5664, "Name": "_RTL_CRITICAL_SECTION", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 40}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5664, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5663, "FieldOffset": 0, "Name": "DebugInfo"}}, "type_references": [{"index": 5663, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 5662, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 5662, "kind": "LF_STRUCTURE", "spelling": "_RTL_CRITICAL_SECTION_DEBUG", "status": "PDB_record_name"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5664, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 18, "FieldOffset": 8, "Name": "LockCount"}}, "type_references": [{"index": 18, "spelling": "Int32Long", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5664, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 18, "FieldOffset": 12, "Name": "RecursionCount"}}, "type_references": [{"index": 18, "spelling": "Int32Long", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5664, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 16, "Name": "OwningThread"}}, "type_references": [{"index": 1539, "spelling": "Void / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5664, "order": 4, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 24, "Name": "LockSemaphore"}}, "type_references": [{"index": 1539, "spelling": "Void / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5664, "order": 5, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 32, "Name": "SpinCount"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 119130, "original_path": "/home/h3/.cache/cargo-xwin/xwin/sdk/include/um/winnt.h", "line": 24367, "module": null, "source_string_id": 119129}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] _LIST_ENTRY; TPI 0x1622
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_LIST_ENTRY", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [5671]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] _RTL_CRITICAL_SECTION_DEBUG; TPI 0x1624
// 原始定义文件映射: PDB_UDT_source_line
// size = 48; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 9, "Options": ["None"], "FieldList": 5667, "Name": "_RTL_CRITICAL_SECTION_DEBUG", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 48}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5667, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 33, "FieldOffset": 0, "Name": "Type"}}, "type_references": [{"index": 33, "spelling": "UInt16Short", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5667, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 33, "FieldOffset": 2, "Name": "CreatorBackTraceIndex"}}, "type_references": [{"index": 33, "spelling": "UInt16Short", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5667, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5659, "FieldOffset": 8, "Name": "CriticalSection"}}, "type_references": [{"index": 5659, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 5658, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 5658, "kind": "LF_STRUCTURE", "spelling": "_RTL_CRITICAL_SECTION", "status": "PDB_record_name"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5667, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5666, "FieldOffset": 16, "Name": "ProcessLocksList"}}, "type_references": [{"index": 5666, "kind": "LF_STRUCTURE", "spelling": "_LIST_ENTRY", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5667, "order": 4, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 32, "Name": "EntryCount"}}, "type_references": [{"index": 34, "spelling": "UInt32Long", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5667, "order": 5, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 36, "Name": "ContentionCount"}}, "type_references": [{"index": 34, "spelling": "UInt32Long", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5667, "order": 6, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 40, "Name": "Flags"}}, "type_references": [{"index": 34, "spelling": "UInt32Long", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5667, "order": 7, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 33, "FieldOffset": 44, "Name": "CreatorBackTraceIndexHigh"}}, "type_references": [{"index": 33, "spelling": "UInt16Short", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5667, "order": 8, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 33, "FieldOffset": 46, "Name": "Identifier"}}, "type_references": [{"index": 33, "spelling": "UInt16Short", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 119131, "original_path": "/home/h3/.cache/cargo-xwin/xwin/sdk/include/um/winnt.h", "line": 24337, "module": null, "source_string_id": 119129}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] _LIST_ENTRY; TPI 0x1627
// 原始定义文件映射: PDB_UDT_source_line
// size = 16; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 5670, "Name": "_LIST_ENTRY", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 16}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5670, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5669, "FieldOffset": 0, "Name": "Flink"}}, "type_references": [{"index": 5669, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 5666, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 5666, "kind": "LF_STRUCTURE", "spelling": "_LIST_ENTRY", "status": "PDB_record_name"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5670, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5669, "FieldOffset": 8, "Name": "Blink"}}, "type_references": [{"index": 5669, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 5666, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 5666, "kind": "LF_STRUCTURE", "spelling": "_LIST_ENTRY", "status": "PDB_record_name"}}}]}
// [PDB UDT 源码行] {"ipi_index": 119132, "original_path": "/home/h3/.cache/cargo-xwin/xwin/sdk/include/um/winnt.h", "line": 1224, "module": null, "source_string_id": 119129}
// === 完整 TPI 结构关系引用记录；原文不转换成猜测的 Rust ABI ===
// [TPI 0x161a] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_RTL_CRITICAL_SECTION", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x161b] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 5658, "Attrs": 65548}}
// [TPI 0x161e] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_RTL_CRITICAL_SECTION_DEBUG", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x161f] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 5662, "Attrs": 65548}}
// [TPI 0x1620] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5663, "FieldOffset": 0, "Name": "DebugInfo"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 18, "FieldOffset": 8, "Name": "LockCount"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 18, "FieldOffset": 12, "Name": "RecursionCount"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 16, "Name": "OwningThread"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 24, "Name": "LockSemaphore"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 32, "Name": "SpinCount"}}]}
// [TPI 0x1621] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 6, "Options": ["None"], "FieldList": 5664, "Name": "_RTL_CRITICAL_SECTION", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 40}}
// [TPI 0x1622] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_LIST_ENTRY", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1623] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 33, "FieldOffset": 0, "Name": "Type"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 33, "FieldOffset": 2, "Name": "CreatorBackTraceIndex"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5659, "FieldOffset": 8, "Name": "CriticalSection"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5666, "FieldOffset": 16, "Name": "ProcessLocksList"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 32, "Name": "EntryCount"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 36, "Name": "ContentionCount"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 40, "Name": "Flags"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 33, "FieldOffset": 44, "Name": "CreatorBackTraceIndexHigh"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 33, "FieldOffset": 46, "Name": "Identifier"}}]}
// [TPI 0x1624] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 9, "Options": ["None"], "FieldList": 5667, "Name": "_RTL_CRITICAL_SECTION_DEBUG", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 48}}
// [TPI 0x1625] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 5666, "Attrs": 65548}}
// [TPI 0x1626] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5669, "FieldOffset": 0, "Name": "Flink"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5669, "FieldOffset": 8, "Name": "Blink"}}]}
// [TPI 0x1627] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 5670, "Name": "_LIST_ENTRY", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 16}}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: winnt.h.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===
