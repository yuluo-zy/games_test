# History 三 writer 身份纠正（B RNE）

本轮完整原生证据纠正旧Ghidra简化表达式/寄存器误映射，不沿用旧完成度结论。目标RNE SHA256 f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03。

PDB helper14143b610的真实fn参数顺序：History、PendingHistoryCommands、ReplayBuffer、HistoryCmd Reader、DebugHistoryMsg Reader、PlayReplayAudio Writer、NewReplayCommandEvent Writer、InterruptTimelapse Writer、AnimateWallBulldozeBuffer。

- Windows函数prologue在140af5b5d取rsp+1a8（arg6），保存rsi；5c2a取Events ptr到r14。140af5e68比较入口byte3==3，等于则跳过音效；不等则140af5e72取得arg6的Tick/Events，140af5c87 slot index<<4，5c8b写EventId，5c8f写byte+8=byte3。因此入口通知是既有PlayReplayAudio{kind}，并非NewReplay。
- arg7在140af5b50由rsp+1b0保存到rsp+68。出队通过byte0!=2/byte1!=4门后，140af6017重新取arg7，604d按8B stride仅写EventId；该NewReplay是unit，在Undo/Redo分派和bounds检查之前发送。静音byte3=3也必须发送它，才能触发真实history_clear_previous_state。
- arg8在140af5bb6从rsp+1b8取，5c14保存为rdi。相反方向的排队命令/explicit清队/instant化分支在140af5df8..5e37按8B stride写InterruptTimelapse，随后5e4d/55清待执行队列。当前新增canonical unit事件，没有伪造timelapse消费者。
- 第四字节仍不传播到ReplayBucket。140af6046 shr ebx,8只留下byte2的低bit给真实generator的DIL消费；高位没有被读取。

独立consumer反证：replay_audio_system140ad5800完整2640B中，140ad5ab6/5ad6索引<<4，5b40 stride+0x10，5b4a/5baf读byte+8，0=Undo、1=Redo、其它DenyInput。由此撤销“PlayReplayAudio可能ZST”的旧疑点；既有kind:u8正确。

生产修复在当前history.rs，同一pure policy产生audio/interrupt effects，实际入口发送它们；cursor真正发unit NewReplay。旧HistoryControlProcessed只作为PlayReplayAudio同类型reexport保留兼容，不产生第二ECS身份。新增两项有限回归：静音load命令仍推进真实空MultiEdit并发NewReplay；相反方向清队只发Interrupt与实际Audio.kind，尚未pop时无NewReplay。父任务统一Cargo。
