# C 版剩余 shabriri 生成绑定

来自原 PDB 的 country-core 和 clutter-icon-pipe OUT_DIR 文件，保留原 `target/.../out/shabriri.rs` 结构与源校验和。共 2 个文件、483 个函数符号/类型条目。

`.rs` 保存函数身份、源码行、原地址及完整可用 TPI/IPI 引用图，含泛型/闭包作用域与编译上下文。同名压缩附档保留逐条行表、内联操作、符号字节和展开类型记录。完整 PDB 复用 `skeleton/_pdb/C/`。

另外 6 个 rhapsody2 生成文件在 `C/rhapsody2/_generated/`；本目录补齐 C PDB 中余下的两个。缺失的参数/返回类型、字段布局和函数体仍明确标注，不冒充作者生成器输出或可运行实现。

核对结果见 `_metadata/verification.json`。原始生成器与 Cargo build.rs 尚未恢复，此目录不合并进 reconstruction。
