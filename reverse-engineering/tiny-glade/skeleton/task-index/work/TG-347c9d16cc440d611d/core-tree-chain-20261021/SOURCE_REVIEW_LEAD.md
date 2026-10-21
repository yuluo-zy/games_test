# 树放置与命令处理候选独立复核

复核者：core_business_lead。已直接读取 producer 和 processor 的完整候选补丁，复核对应 B 完整函数原字节包、processor mode/scale/fallback 分支及 History::add 缺录制失败分支。未运行 Cargo、测试、GPU 或原程序；仅作源码与原证复核。

准许 root 配对整合两个文件：

- `ui_place_tree.rs`：以真实 storage index 初始化 pinned fastrand1.8 状态，保持 f32→i32(0..10)→f32 顺序、原乘加次序、季节与 Halloween prefab 分支、原布尔真值，并先提交 Insert 后记录同一完整数据的 History。原 `History::add` 在缺输入录制时沿 `140a84ef7`→`140a8525b`→`panic_fmt` 失败，因此 current Result 的 `.expect` 是明确适配，而非新造失败。
- `process_tree_cmds.rs`：修正主题读取与 missing-prefab 的同一会话 caller、原 mode 优先级、olden 裸树 MAXSS NaN 分支、原声音 flags 与替换树的声音先于回收顺序。保留现有 Move/Delete 失败及按动画删除路径，不额外改状态所有者。

新增固定 seed 期望值来自独立 Python 转录原 WY 整数/拒绝采样与逐步 f32 舍入；未执行 pinned Rust 或目标指令取得。root 的 Rust 测试应作为第二条实现对照，不能把 oracle 静态输出视为原运行结果。

Producer before=`e924766701718bd60140a11c3baa0f8cae04b126027f28b223d948f9c9963b82`，after=`a53080e596af0aab1a123d0c5d242aff714027128a1a6b3bb32ccde5170e0d90`。Processor before=`1e09c4eaf42868a7f1d810a575c52544f43886b45fda18ec686c5d9806f42989`，after=`00eda7147cac0fd44818634875caa395f2ba60efc2b723cca0449b8456405d27`。

集成不能只应用 producer 后就保留 processor 旧主题输入。UI cursor icon 闭包、共享 signifier builder 与 DEV 全局门仍独立追踪；本候选不宣称整套 UI 或最终渲染完成。
