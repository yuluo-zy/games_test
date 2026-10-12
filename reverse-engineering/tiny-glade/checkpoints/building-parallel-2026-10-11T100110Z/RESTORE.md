# 恢复快照

project-source-evidence.tar.gz 保存当前项目相对目录与源码/证据。建议解压到新的空目录保留当前工作区，随后阅读 CHECKPOINT_CURRENT_IMPLEMENTATION.md 与 tracking/parallel-implementation-conflicts。manifest.json 是归档文件清单与保存哈希，不是业务验收。原始EXE/PDB/资产与Ghidra工具数据库仍使用原外部路径。

恢复时先统一解决接口和 Cargo.lock，再按用户指示构建及验证；不要把快照中的阶段4报告当作本批新增源码已通过的证明。
