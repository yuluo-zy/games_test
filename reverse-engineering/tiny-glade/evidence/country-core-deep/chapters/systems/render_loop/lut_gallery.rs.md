# systems/render_loop/lut_gallery.rs

> 文件作用：摄影模式 LUT 画廊：`LutGalleryPlugin::build` 注册插件，`render_photomode_lut_gallery` 渲染 LUT 缩略图画廊（每项 `lut_gallery_thumbnail` 纹理）。

函数总计 4：业务 2 / 内联无独立符号 1 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::lut_gallery::impl$0::build` | 0x1408b04c0 | 35 | 插件 build：注册 LUT 画廊系统/资源（小插件，返 self；wp5 未单列） | I（插件 build 结构，无独立强证据） | render-loop\decomp\decomp-0x1408b04c0.txt |
| `country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery` | 0x1408b0560 | 55 | 摄影模式 LUT 画廊渲染：串证据 `lut_gallery_thumbnail`；读 GradingLut 列表渲染缩略图到画廊 UI 目标 | O | render-loop\decomp\decomp-0x1408b0560.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `startup`（行 41）— 启动时建 LUT 画廊资源（生命周期控制）

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 1 项（逐 LUT 缩略图构造闭包）

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

### `render_photomode_lut_gallery`（0x1408b0560）—— 逐格 LUT 缩略图
处理链（O，B_s_h S-B 组）：
1. **demo 开关静态字节 0x143634d90**；AppMode==(0xa,1) 门控；预览槽 Arc 缺失/槽内 None 直接返回；
2. pass "LUT thumbnail"；逐格 `zip(lut 列表, positions)`；
3. 绑定 3 张纹理：预览（`0x11ba5697cc40b3dd`）、磁盘（`0xA647E80AF4E9395F`→`-0x3f4f…`）、`NameHash(name)`（`0x0422b14c27be4803`）；
4. **uniform 序**：extent、alpha=1.0、pos、size、highlight(=flag&&selected==i)、is_current(名字相等)、is_first(i==0)；
5. state byte0&=0x7f、byte3=(b3&0x9f)|0x40；每格 `draw_no_vao(6)`；最后 render。
- 资源：LutGalleryState（0x48B Arc<Mutex<0x34B>>）、当前 LUT 条目表、LUT 列表；`startup` 加载 `/lut_gallery/thumbnail.raster.hlsl` = `lut_gallery_thumbnail`。

## 关键事实

- 串证据：`lut_gallery_thumbnail`（每张 LUT 的缩略图纹理名）。
- 跨域：依赖 `resources/grading_lut`（audio-misc 域已覆盖 GradingLut）。
