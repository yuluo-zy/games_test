# 全组合对齐叶合同独立复核

复核人 core_business_lead，2026-10-21。独立逐读14080c360完整733B及清理范围，合同可进入source candidate。

14080c3bb取rest且stride32，3ce..3d6候选stride24；3da初始化-f32::MAX；40f..42e的两乘积cross与setbe包括unordered，mask为2或4；432..449真实递归；461再467固定(current+child)+penalty顺序；470/474只ordered strict greater取代；49d..542接管child Vec并在首部插入当前24B(id,kind)；565..57c空输入返回空/+0。非法tag5a7 panic。空候选返回空/-MAX不在父级跳过，tie保持首个，不得改为贪心/排序/局部最高分或忽略NaN比较。

验收须覆盖多邻居全局选择、mask重复-10、tie、空候选和非finite，所有执行由root统一。该批准限必要叶，30KB×2请求producer仍独立未闭合。
