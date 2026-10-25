# systems/render_loop/session_selection/mod.rs

> 文件作用：会话选择界面渲染（邻域）：SessionSelectionPlugin 注册；thumbnails_startup / render_session_selection_previews 渲染存档缩略图（从磁盘 screenshot.jpg + meta.json 生成 painterly 缩略图池）。

函数总计 15：业务 3 / 内联无独立符号 4 / 派生胶水 8

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::session_selection::impl$0::build` | 0x1408f68c0 | 39 | 插件 build：注册资源（FUN_140b67dd0）+ 系统（FUN_1415451c0）小插件 | O | round2\decomp-0x1408f68c0.txt；render-loop\decomp\decomp-0x1408f68c0.txt |
| `country_core::systems::render_loop::session_selection::thumbnails_startup` | 0x1408f6900 | 272 | 缩略图启动系统：串证据 `/session_selection/new_glade_thumbnail.raster.hlsl` + `session_selection_new_glade_thumbnail`、`/session_selection/thumbnail_painterly.raster.hlsl` + `session_selection_thumbnail_painterly`（磁盘→纹理加载） | O | round3\decomp-0x1408f6900.txt；render-loop\decomp\decomp-0x1408f6900.txt |
| `country_core::systems::render_loop::session_selection::render_session_selection_previews` | 0x1408f6c00 | 336 | 会话缩略图渲染系统（120KB）：串证据 screenshot.jpg / meta.json / assets/gladestate / loading_background.jpg；`assertion failed: idx < self.texture_pool.len()`（纹理池索引边界）；`assertion failed: backed_range.len() <= THUMBNAIL_POOL_SIZE.min(save_list.sessions.len())`；painterly 缩略图光栅 | O | round2\decomp-0x1408f6c00.txt；render-loop\decomp\decomp-0x1408f6c00.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `ThumbnailTextures::extract_thumbnails_to_display`（行 72）— 提取可见缩略图范围
- `ThumbnailTextures::find_backed_thumbnail_range`（行 127）— 查找已落盘缩略图范围（`backed_range.len() <= THUMBNAIL_POOL_SIZE.min(…)` 断言）
- `ThumbnailTextures::assign_pool_indices`（行 149）— 分配纹理池索引（`idx < self.texture_pool.len()` 断言）
- `refresh_screenshot`（行 235）— 刷新 screenshot.jpg 回读

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`default` 等 8 项

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

### 1. `thumbnails_startup`（0x1408f6900）
4 着色器 + **6 张 512×512 格式 0x2b mip1 纹理**（失败仅记日志）：`/session_selection/{new_glade_thumbnail, thumbnail_painterly}.raster.hlsl` 等。

### 2. `render_session_selection_previews`（0x1408f6c00，120KB）—— 池窗口算法
- **窗口**：`start = floor(min(len, max(0, scroll)) - pool/2 + 0.5)`、`end = min(len, start+pool)`；断言 `backed_range.len() <= THUMBNAIL_POOL_SIZE.min(save_list.sessions.len())`；
- 锁内 `mode_tag==1`（assert_eq）、`active!=0`（否则 `panic!("TODO")`）；pass D 按截图路径逐组件匹配复用池项，pass E 取第一个未分配项（`expect("thumb pool entry")`），新分配项 modified 复位 FILETIME 1970；`idx < self.texture_pool.len()` 断言；
- **刷新**：metadata("Reading metadata") → modified("Checking modification time") → 比较（有符号 `>=` 跳过）→ 解码("Loading the image") → `update_texture`("updating texture")，成功写回 modified 并 debug "Loaded session thumbnail image"，失败不更新 modified 且按 `"module_path! ()$crate :: Level :: Error"` + 消息去重记 error（消息 `Failed to refresh screenshot {tail}: {err}`）；
- **绘制序**：背景(3 顶点) → "新建"卡(槽 0，6 顶点) → 全部会话卡 thumbnail 着色器(6 顶点) → 全部会话卡 painterly 着色器(**0x300 顶点**，多绑一张磁盘纹理)；位置 `(slot*spacing+origin.x, origin.y)`；高亮 `state&1 && slot==selected`；bool uniform `(index>=3 && demo) || session.flag`；窗口外会话用占位磁盘纹理。
- 常量 THUMBNAIL_POOL_SIZE；磁盘约定 screenshot.jpg/meta.json/assets/gladestate/loading_background.jpg。

## 关键事实

- 常量：`THUMBNAIL_POOL_SIZE`（纹理池容量，配合 `save_list.sessions.len()` 上限）。
- 磁盘约定：`screenshot.jpg` + `meta.json` + `assets/gladestate`（会话目录）、`loading_background.jpg`（加载背景）。
- 跨域：依赖 `resources/session.rs`（session 列表）与 painterly 缩略图管线（`/session_selection/thumbnail_painterly.raster.hlsl`）。
