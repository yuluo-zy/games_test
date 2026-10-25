# 原 AllocId 相同值复用与 usage gap 补充

追加 [native_same_id_oracle.py](native_same_id_oracle.py) 和 [native-same-id-oracle.json](native-same-id-oracle.json)：21 项原指令案例零差异。原 791 结果文件SHA复验保持不变，未改生产 Rust。

16 项验证以 frame1/2/6/MAX 分别检查：raw AllocId=0（index0、generation0）复用后，slot.last_used非零时原validator接受并立即写current_frame；请求generation1而slot为0、请求0而slot为1均拒绝；首项id0合法、后项generation1不匹配时仍保留首项last_used更新。原无额外自增generation/epoch，所以同rawID历史引用可重新变有效，这是原合同，不能用新counter强化拒绝。

5 项完整原usage helper `0x141800040`（PE主体711字节）验证 len3/cap8 请求slot5：slots3/4的u32 last_used与u16 generation都补零、slot5写current_frame与generation0、len变6，slots0–2/6–7与每slot pad2保持。容量足够，未执行allocator/realloc或GPU，原padding不是合同字段，JSON只记录诊断值。

validator仍只执行原render内未改prefix，并用既有方式在其后继点标记reuse/regenerate/bounds分支；这不是整游戏/真实etagere分配运行。相同ID是否由实际etagere再分配产生由父集成测试确认，这21项只证明原slot写入与validator对该相同值的精确行为。
