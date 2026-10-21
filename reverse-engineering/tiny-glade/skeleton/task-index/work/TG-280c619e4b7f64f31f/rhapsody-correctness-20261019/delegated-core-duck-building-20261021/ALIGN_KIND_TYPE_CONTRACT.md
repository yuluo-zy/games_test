# AlignKind 最窄必要类型修正

原 B 完整原证位于 TG-f768c2955f5ef10ede/core-duck-building-20261021/helpers-native-pack（RNE f507/PDB2d968，report5项ok，未执行target）。preferences14080bf9a分配72B/align4，14080c0b9..107三项起点0/18/30，+14/+2c/+44写独立score；best-neighbor14080c4de/4e4读kind前16+末4B，14080c527写ID@0，52f写kind16B@4，537写kind末4B@14，输出pair stride24。故值本身20B/align4；旧24B把外层score/pair错算入值。

独立复核：core_business_lead在2026-10-21消息明确自行读两个ASM，确认4+20 pair和score归属，批准本源删除pad_14、NATIVE_SIZE20、align4及迁移两本源字面量/错误测试。保既有position_offset/outgoing_dir数学字段，不据此声称请求图/装配/缓存链已恢复。historical claim rhapsody-correctness-20261019不变，root经lead窄委派本合法work。
