# 原作 shader oracle bin（B 版本镜像）

来源：`/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/compiled-shaders/`（404 个 `.bin`，SPIR-V），2026-10-13 镜像入仓库。

- 只读 oracle：本目录用于 dxc 重编译结构指纹校验与重建对照；不修改。
- 每个 `.bin` 由 dxc `-Zi -Qembed_debug` 编译，嵌入源码路径、名字与编译命令（见 `../catalog.json` 的 sha256 与 `../toolchain-embedded-debug.md`）。
- 命名编码：`<pass>.<cs|raster>.hlsl_<entry>[_<variant>].bin`；raster 双变体（`b903e43ffb3da915` 全 / `f0adff76827e73ff` 最小）。
- 与重建源码对应关系见 `../catalog.json`、`../embedded-shader-source-paths.txt` 与 `reconstruction/shader-source/shaders/`。
- 原始发行文件仍保留在原路径；本目录是提交入 git 的验证用副本。
