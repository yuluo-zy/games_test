# Ivy最窄canonical数据API独立源码复核

2026-10-21 tree_commands。核准当前候选进入root整合；未改candidate/claim/生产。

before ec3d7c77e4d8bed26b45ce0951d0b65401f0d88a4b652d25eea2c874836e4ab7 → after 5bc8b4f4a82f1d0d3a49b8b34a8f49089e1ee2dc5e2776eb2166aee4109f7dbf，patch 84485b4909a80bec1da91c7e934a60771bb2618b9d468e690a401cdc3c98bf57，实际三SHA全部重核一致。

原完整1655B两分支a62/da6实际ID→subtype，a6c/daf rank exactkey，a86/dc9 required lookup，再读+30、+3c-+38及+0/+1，已在合同review直接对原asm复核。候选SystemParam只借真实两个owner；view Copy不克隆表、Arc或TriangleVec，私有helper保ID失败先于mesh失败、相同rank不fallback。width+0及(top-bottom)+1保原操作次序，没有引入AABB投影或Info.type替subtype。

测试确用真实primitive shape+box TriangleVec+typed显式None outline，rank2/非零bottom/rotatedfootprint度量域抓逻辑宽高，不是默认shape壳；actualID17映射subtype3、rank1/ID18及碰撞表清空失败均覆盖，源码无mesh clone。Arc返回后计数只辅助，不应当成瞬态clone检测；零复制由实际源码借用证实。旋转footprint是度量域夹具，非完整旋转碰撞物理一致性验收。

本批准严格不包含原ivy事件流、cached坐标、删除传播等已声明旧算法差异；现遍历与raw位置未因本接口补丁获完整恢复结论。未Cargo/rustc/GPU/原程序执行；root负责配对四表+box后实际测试。
