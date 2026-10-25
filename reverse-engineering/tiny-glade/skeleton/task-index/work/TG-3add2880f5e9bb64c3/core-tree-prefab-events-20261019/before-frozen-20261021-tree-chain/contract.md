# process_tree_cmds 同链 consumer 合同（待独立复核）

完整 B 原字节在 ../ui task core-tree-chain-20261021/process-native/report.json；process @142267430 长4758 B、despawn @1422686d0 长751 B，identity绑定原B RNE/PDB，均 status ok。

## ABI 与权威输入

- PDB typed wrapper的原参数含 ActiveSession，未含 LoadingCompletionState。process @142267973..9d6 解 Res<ActiveSession> 后取 +0x58 季节byte，写入 RBX。后续 14092e6a0 / 140a9fa70 / 140ae8320 均 RCX=RBX，分别 is_flowery / is_olden / is_winter；后两有 ActiveSession PDB inline，绝非 UI拖动/加载readiness/存档corrupted。
- 输入是既有 InsertTreeCmd 字段 id(index u64)、variant(i32)、bool、position、scale、rotation、prefab_name；不新增共享字段或资源。此前所谓flags/height缺口其实+0x2c是variant，+0x30是bool，+0x40是EventId。

## Insert 完整分支

1. original_scale 上 inverse_lerp(.5,.9) 得r，并计算 duration=r*.75+(1-r)*.55。原 scale NaN 保留，不改变audio r。
2. 裸树名称精确集合large/naked1/naked2/naked1_no_mushrooms/naked2_no_mushrooms；与最终fallback名无关（比较原事件名）。
3. flowery_extra = session.is_flowery() && (variant & 3 != 0)，@142267aaa..abc。naked&&olden 时 scale使用MAXSS(scale,.7)：NaN时结果.7；其它scale不变。@be3..c0c。
4. Commands::spawn_empty，再按 TreeComponent(index,bool)→TreeAnimation(duration,0,0,false)→TreeMoveBend(x,z)→TreeAmbience(position+Vec3(0,10*adjusted_scale,0)) 顺序入队。不造新状态表。
5. spawn mode：naked&&olden→3；否则bool=true→0；否则flowery_extra→2；否则→1，@142267dcd..ded。
6. prefab_name=MISSING_TREE_PREFAB_NAME 时调用同一 random_tree_prefab(index,&ActiveSession)，@142268146..14d；其它名克隆。只需传 olden/halloween/winter/autumn，无readiness/drag等资源。
7. SpawnPrefabCmd 保持原 position/rotation/variant/one=1，adjusted_scale；flags16全0是原版 @142267ea8 MOVUPS XMM14零值，并非缺失实现；不得发明 flags。
8. storage.insert(index,new_entity) 后水样→PlayTreeSoundCmd(kind0,in_water,naked&&(olden||winter),r,position)；如替换旧entity，再日志→DespawnPrefabCmd(old)。当前despawn先于sound需移后。@142267f72..142268141。

## 保留失败/删除/历史边界

- Move→Insert→Delete顺序；Move lookup missing仅警告，lookup成功但真实 PrefabInstanceData不存在时panic；不以Transform fallback掩盖。
- Delete remove(index)后entities.contains包含reserved entity；live placement query失败保留panic；先排 TreeMarkedForDeletion，再 PlayTreeSoundCmd(kind2)；不存在条目/实体分支原样skip。
- despawn仅marked && TreeAnimation.progress严格<.01；等于/NaN不回收。保持原跨帧回收流程。
- replay_tree 已使用完整 index/name/position/scale/rotation/variant/bool，原样保留；本轮producer byte1对应 bool恢复true（String clone无bool转换）。

## 最窄候选

只改当前process源的主题读取、fallback caller、mode、MAXSS NaN及sound/replacement side effect顺序，删除无关readiness参数/import及过期gap注释；其余Move/Delete逻辑保留。单元测试在同候选当前文件追加，覆盖五主题、两flag值、variant0/1/4/负值、missing prefab、naked olden/winter声音、NaN clamp、不依赖readiness。

## 验收限制

root执行 system-tree tests/生产闭包；本child不编译。待 producer、consumer同审通过后可集成编辑命令分支；Ui seed_cursor_icon闭包和DEV override仍独立缺口，不能称UI渲染整体完成。
