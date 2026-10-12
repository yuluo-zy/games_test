# Tiny Glade Blender 可编辑素材库

已构建：**原始网格保留 + 可编辑结构重建**。适用于 Blender 5.2.2 LTS。

## 打开与开始编辑

1. 双击 `打开素材库.command`，或打开 `blends/00_StartHere.blend`。
2. 在展览中选中一个 `EX_ASSET_` 实例；原入口示例场景中则选 `PREVIEW_` 实例。
3. 按 **N** 打开侧栏，进入 **TG 素材库**。
4. 点击 **制作可编辑副本**。网格、控制笼、骨架会复制为独立数据，原有预览实例隐藏，资产定义保留。
5. 用 **另存为** 将你的修改保存到 `user_projects/` 或自己的工程目录。

也可将编辑器切换为 **Asset Browser**，选择 **Tiny Glade Editable**。资产以集合为单位，可拖入任何工程，再用侧栏制作可编辑副本。

如果应用安装插件时已经在运行，关闭并重新打开 Blender，确保 TG 素材库插件和新素材库配置加载。无需允许工程脚本自动执行。

## 可以如何修改

| 目标 | 操作 |
|---|---|
| 改网格细节、增删面 | 选择 `SM_` 网格，Tab 进入 Edit Mode |
| 改整体轮廓、弯曲、局部拉伸 | **编辑整体形状笼**，移动 27 个 Lattice 控制点 |
| 改现有树枝方向、弯曲和长短 | **调整现有枝条**，在 Pose Mode 中旋转/移动/缩放骨骼 |
| 增加枝条 | 在 Pose Mode 选一个骨骼，点击 **新增可编辑枝条**，Tab 编辑曲线 |
| 改新增枝条粗细 | 侧栏 **枝条粗细** 或几何节点修改器的 **Branch Radius** |
| 删除新增枝条 | 删除该独立曲线对象，不影响原树网格 |
| 删除已有枝条 | **查看此枝条影响的顶点**，核对范围后手工删改；权重影响范围不是准确枝杈边界 |
| 编辑窗户的框/玻璃/帽檐 | 使用 `KIT_` 组合资产，各部件独立，共用形状笼 |

**5 套组装树**的树干与树冠共用形状笼和骨架。树冠会随权重变化；新增枝条不会自动生成匹配的叶冠，可以复制叶片/树冠几何并手工调整。生长、自动分叉和原作风动系统尚未重建。

## 数量与分类

- 三版合并 **487 个同名资源项**；保留几何/属性差异后 **562 个资源及变体**。
- 加入 **5 套树组合**、**6 套窗户组合**，共 **573 个集合资产条目**。
- **405 个形状笼**、**25 个枝条骨架资产**（含 5 套树组合）；骨架来源为 20 个网格的几何推断。
- **392 项贴图**解码为 PNG，数组贴图按层保存；一项体积 LUT 原样保留。
- 动画文件包含 31 个羊网格资产；30 个编号帧另建序列预览，`delighted` 单独保留。
- 每个集合资产带几何缩略图、分类目录和来源标签。

| 文件 | 内容 |
|---|---|
| `blends/00_StartHere.blend` | 573 项全素材展览、9 个分类视图、原 12 项编辑示例；相对路径链接分类工程 |
| `blends/01_Vegetation.blend` | 树木、树桩、枝条、灌木、5 套组装树 |
| `blends/02_Plants.blend` | 花草、藤蔓、水生植物 |
| `blends/03_Architecture.blend` | 建筑构件、门窗、瓦片、木板、6 套组合窗户 |
| `blends/04_Props.blend` | 道具及跨版本变体 |
| `blends/05_Terrain.blend` | 地形区块、石头、水面等 |
| `blends/06_Animals.blend` | 静态动物外形与形状控制笼 |
| `blends/07_Other.blend` | 其他资源 |
| `blends/08_AnimationFrames.blend` | 羊的网格帧与序列预览 |
| `blends/09_Helpers.blend` | 碰撞、轮廓、交互、粒子发射点等辅助数据 |

每个分类工程包含：

- **Gallery**：预览实例，适合浏览并复制编辑。
- **AssetDefinitions**：可编辑资产定义，顶点坐标已转换为 Blender Z-up。
- **SourceArchive**：精确保留的原始网格。坐标不改，仅用对象旋转方便查看。
- 建筑文件另有 **WindowKits**；动画文件另有 **Sheep_MeshSequence_Preview**。

## 保留了什么

- 原始模型坐标、原始索引和全部版本 JSON 文件，以 SHA256 内容寻址归档。
- 原始顶点属性以 `SRC_` 前缀保留，例如法线、UV、顶点色、prim_center、appear_pos、age、wind_bend 等。
- Blender UV 层与浮点顶点色层；原始属性的额外备份不依赖这些显示层。
- 文件名、版本、哈希、变体和双面显示提示。
- 旧版具名纹理、贴图处理 RON 配置、三版 prefab/glade 配置，以及新版 foliage package 的原始文件。

可编辑网格采用 `(x, y, z) → (x, -z, y)`，仅旋转坐标约定，不推断真实世界比例。部分面绕序根据源法线调整；原始绕序在源网格与归档 JSON 中保留。

## 重建的边界

- **树骨架和权重是近似重建**：对实体网格焊接位置作分析，沿表面路径推断控制骨骼，再按距离绑定；没有破坏原始拓扑。复杂分叉和局部变形可进一步精修权重。
- 中距离树干/树冠配对来自 prefab 配置；窗户部件按同名局部坐标配对，原作附着/变形系统未恢复。
- 材质用于编辑预览。部分保留顶点色或具名贴图；树的颜色通道可能是着色器参数，树叶预览进行了近似适配。没有声称恢复原作完整渲染效果。
- BC7/BC4/RGBA 贴图格式由代码、尺寸和数据长度推断并校验。色彩空间与游戏通道语义仍需按实际材质核对。
- 原始建模历史、四边面控制网格、原作骨骼/控制器、程序化建筑生成规则没有恢复。
- 动物目前可改静态形状；羊的序列是独立网格切换，不是动画绑定。24fps 仅用于预览，不是恢复的原作时序。
- 点资源和轮廓保持为点/边，不人为补成实体模型。

## 验证

`reports/verification.json` 记录实际 Blender 验证，全部通过：

- 九个分类工程重新打开；562 个网格条目检查。
- 1,924 个原始属性缓冲逐项比对；原始坐标误差为 0。
- 可编辑网格静止时最大误差约 0.000006 个源单位，来自浮点变形计算。
- 在临时独立副本上，实际测量 Lattice、枝条姿态、树冠跟随、新枝条曲线与半径的变化。

`reports/manifest.json`：全部版本与贴图清单。

`reports/build-report.json`：逐资产控制器、属性与材质状态。

`reports/tree-controls.json`：推断的树木控制结构。

`previews/library-overview.png`：实际 Blender 渲染的入口示例。

## 全素材展览与高清渲染

入口工程默认打开 **AllAssets_573_全素材展览**，全部 573 项素材按 9 类摆放。
可切换到 **Exhibit_01_Vegetation** 等分类场景查看，原入口示例场景保留。
选中 `EX_ASSET_` 集合实例后，同样用 **N → TG 素材库 → 制作可编辑副本** 开始建模。

`renders/all-assets-overview.png` 是高清总览，`renders/01_Vegetation.png` 至
`renders/09_Helpers.png` 是分类渲染。打开 `renders/查看渲染.html` 可以浏览全部图并放大。

展览按每个条目的包围盒统一展示大小，并非统一原始比例。
辅助资源的原始集合实例保留在展览里；渲染使用独立的线框和点标记，未改动原始点/边/面。
编号、资源 ID 与位置对照在 `reports/exhibition-index.csv` 和 `reports/exhibition-placement.json`。

`scripts/create_exhibition.py` 在原入口上增加展览；`scripts/render_exhibition.py` 渲染已保存的各个场景。
展览前入口备份在 `reports/backups/00_StartHere.before-exhibition.blend`。
恢复时将该备份复制回 `blends/00_StartHere.blend`，再打开，以正确解析相对路径。

## 重新安装与再生成

插件源文件：`addon/tinyglade_asset_tools.py`。安装脚本：`scripts/install_library.py`。当前已安装并注册到 Blender 资产库。

整个目录可复制到其他位置；重新运行安装脚本更新资产库路径。源文件和贴图已在目录中归档；模型用到的预览贴图已打包进分类工程。入口工程以相对路径引用分类文件。

再生成流程：`prepare_sources.py` → `reconstruct_tree_controls.py` → `build_library.py` → `enhance_library.py` → `make_thumbnails.py` → `finalize_assets.py` → `verify_library.py` → `create_start_project.py` → `create_exhibition.py` → `render_exhibition.py`。

再生成会覆盖本素材库的分类工程。自己的模型修改请保存到独立工程；请勿把独立编辑成果只留在将被重新生成的分类文件中。
