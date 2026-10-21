# lazy flag 合同独立复核（duck，非source作者）

结论：批准仅合同范围的canonical字段/getter/force mark/reset清0候选。未执行Rust/Cargo/目标，也不验收外部invalidate三邻接完整业务。

独立读原full producer4290B、reset2322B与5条inline store包：

- stride110的bucket起end-110包含两个u32 key，value起end-108；故end-8==value+100。真实consumer为我已完整核的interpolate1407d025d compare此byte，026a零→0756复用；非零→0270重建。不是+101或全局dirty。
- 第一置1前140b3788a检查表len0→379d0跳过；37909/3790f顺序比对两个u32键，全部匹配37914才跳379c3写byte1。未命中空control3793c跳379d0。无insert/default或reset调用，missing no-op成立。
- 第二/第三分别37c10、37df0，在各自相同typed HashMap探测键命中分支。37bfc/37ddc遇emptycontrol直接跳写后37c16/37df6，保missing不创建。写入前后没有geometry清除或globaldirty写。
- 三条store的PDB最内帧皆`StairsVisualState::force_rebuild`，**store精确source Line134**（132/133可为函数/入口行，合同不要把store写成133）；外帧invalidate92/86/89分别不同真实caller。不是62KBassemble直接写。
- reset140aa8e26 word0同时清+100/+101；fresh140aa9067同word0。+101无本轮producer/consumer不能猜成另一bool业务值。只恢复+100字段与原Default/reset0符合最窄授权。

现代getter/privatebool不增加镜像权威；force_rebuild_with保adapter可转发mark而不调用reset，原buffer/allocation/byte_map/globaldirty保持。reset_with自身副作用尚不完整，须保现有已恢复owner行为，不能以新增flag删除清理或声称全部reset已还原。

须source测试保原allocation身份/内容、已存在/不存在entry、原globaldirty两值、reset前后；仅setter测试不够。候选后还需source review与root实际编译。合同本身与原5条store/三命中分支一致，允许开始候选。
