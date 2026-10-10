# Tiny Glade 原作发行资源基线

证据编号 `TG-RES-BASELINE-20261010`。所有原游戏文件只读，未启动原作。本轮复用 `D:/game/TinyGlade_逆向初查/initial-analysis.json` 中已有 E05 资源数量及 E07 shader 格式检查；新清单数量和体积均与该基线吻合。完整记录见 [resources-summary.json](./resources-summary.json)。

| 目录 | 文件数 | 总字节 | 主要实际格式 |
|---|---:|---:|---|
| assets | 966 | 2,022,411,165 | 627 JSON、91 RON、37音频bank、36snapshot及字体/图像等 |
| assets-src | 16 | 5,752 | 全部为纹理构建描述 assets.ron |
| compiled-assets | 389 | 311,687,779 | 388 texture容器、1 RON目录表 |
| compiled-shaders | 404 | 13,562,443 | 带嵌入SPIR-V的封装bin |
| build-info | 3 | 489,664 | Cargo.lock、manifest.json、toolchain-info.json |

整个指定目录递归文件清单中没有 `.rs` 或 `.hlsl/.glsl/.wgsl/.vert/.frag/.comp` 源文件。因此这里是附有 PDB、构建信息和资源描述的发行包，不能认定为完整 Rust/HLSL 源码工程。compiled shader 文件名中的 `.hlsl` 是名称的一部分，实际扩展名为 `.bin`。

全部 627 个资源 JSON 均已成功解析；其中 483 个 mesh JSON 含 attributes 和类型化 buffer，另有 1 个 brick.glb。例如 brick.json 有 600 个位置顶点、900 个索引、is_bevel 属性；roof_tile.json 和 window/decorator/collision/outline 族可直接调查。资源只提供数据或几何原语，不证明轮廓、拼接或装饰适配算法。

`assets/nani_meshes.ron` 的文本明确列出 SolidVertexColor、SolidVertexColorWindow、SolidVertexColorWindowGlass、SolidVertexColorBricks、GothicWindowBricks、RoofTile、Plant、Flower、Bush 等 subset，以及移除/增加属性的描述。这是格式/渲染分桶调查入口；本轮仅读取文本，没有执行其处理器。`assets-src/textures/roof/assets.ron` 直接描述 gamma Linear、resize 1024×1024、mips Full、roof_tile_damage compression Grayscale；这是纹理构建配置，不是 shader 源码。

glade 主题配置可读：summer/trees.json 把 prefab 接到 placements，plants.json 把 mesh/shader 接到 placements。tree/default.ron 分开描述 canopy/trunk，含 extract_canopy/extract_trunk、shader、rt Aabb、audio trigger。TOD default.json 列出按 time_of_day 采样的 sun_sky/fog 参数。它们能帮助追踪参数消费者；距离层阈值、采样插值、随机散布与编辑抑制均未证明。

Cargo.lock 的直接记录包括 Rust/Bevy ECS 0.16.0，rhapsody→ash 0.38.0+1.3.281 / ash-window / shader-compiler，country-core 与 system-roof/system-wall-constructor/system-decorator 等包，libfmod、rapier3d、ron、rkyv、serde。toolchain-info.json 给出 Rust 1.86.0、LLVM 19.1.7、目标 x86_64-pc-windows-msvc、opt level 3/debug true。依赖存在不等于每个运行路径都使用它；wgpu-types 存在也不证明采用 Bevy 默认渲染器。详见 [dependency-evidence.json](./dependency-evidence.json)。

既有 E07 静态检查在全部404个bin中找到了SPIR-V魔数，并验证第一段模块的指令边界；保留结构/字段名可作为 GPU 数据布局的线索。尚未验证所有嵌入阶段、完整SPIR-V语义或GPU执行。texture/snapshot/bank的完整容器解码未实施。

本轮新增价值最高的证据是 33 份 starting-build 的真实编辑历史。已按用户最新范围转向原作结构还原，没有设计/修改其他工程。详见 [原作历史命令发现](../../reconstruction/history/findings.md)、[观察schema](../../reconstruction/history/observed-history-schema.json)、[样本核验](../../reconstruction/history/sample-verification.json)。

原作调查优先链路：历史tag/schema和版本→原作反序列化/应用函数→墙与屋顶领域状态→构造器→实例buffer/shader；装饰target和dst应作为一条独立链路核对。仅当缺少状态或资源格式确实阻塞这条链路时，再解码快照/texture容器。
