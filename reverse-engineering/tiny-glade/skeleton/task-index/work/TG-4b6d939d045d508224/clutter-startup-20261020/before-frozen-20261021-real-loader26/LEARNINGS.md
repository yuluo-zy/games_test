# 本轮恢复经验

1. 共享claim里保留的完整原证常已能闭包，但旧生产仍可留generic host/None壳。不要把既有合同当实现进度；先核生产caller与typed输入，再复用body/callback/rank-table进入源码。
2. 名字同样不能证明算法入口恢复：box_trigger_volume原入口width,height,depth,Transform，旧同名实现只有inner indices-map。原caller ABI和完整body直接暴露这一缺口；应修唯一原文件入口，而非在新producer复制算法。
3. switch反编译遇坏指令会截断17386B真实body。保留完整原字节+PDB绑定，直接解26entry相对跳表、七张rank路径表与format的(pointer,len)，可恢复确切分支和特殊别名，不靠名字扩展。
4. 四表类型必须真实拥有Vec<Triangle>、same Arc<RaycastMeshData>、Rectangle3d，不能由bbox猜新的mesh。原same-insert clone triangles但共享Arc；独立interaction则新Arc。缓存或统一bbox会掩盖原物理与交互差异。
5. 当前安全API若把原ownedVec改为slice并内部复制，producer应借用原Mesh片段，避免再克隆一轮。记录复制所在层而非用字节布局脚手架增加性能损耗。
6. 集成测试资产赋各path唯一几何中心，再用独立冻结原矩阵核每key和shape，可发现同族wrong path/interaction复用错误；只断言52或34数量抓不到错误绑定。
7. UTF8切片第7字节、outline补首点重复、rank3凸包失败、demo guard和先watch后add都属于真实失败/副作用顺序，不应被starts_with、dedup、bbox fallback或ready marker抹掉。
