# resources/window.rs

> 文件作用：**窗口尺寸/光标资源（WindowSize 头 8B=Vec2 f32×2）。**

函数总计 7：业务 6 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::resources::window::WindowSize::new | 0x140949bf0 | 33 | WindowSize::new：窗口尺寸构造 | I |  |
| country_core::resources::window::WindowSize::set | 0x140949c00 | 41 | WindowSize::set：窗口尺寸写入 | I |  |
| country_core::resources::window::WindowSize::as_vec2 | 0x140949c10 | 45 | 0x1400885d0：mov rax,[rcx]——头 8 字节直接当 Vec2 返回（重建 u32 模型错） | O |  |
| country_core::resources::window::WindowSize::as_dims | 0x1400885d0 | 50 | WindowSize::as_dims（PDB 名 as_vec2 同址）：mov rax,[rcx] 头 8B 直接返回（f32×2） | O | audio-misc\decomp\decomp-0x1400885d0.txt, wall\decomp\decomp-0x1400885d0.txt |
| country_core::resources::window::WindowSize::aspect_ratio | 0x140949c20 | 53 | PDB 名挂在 0x140949c40 上但该址=静态析构注册（唯一引用 0x14368E2FC DATA 非 call）；真实体 U | U（错位） |  |
| country_core::resources::window::AvailableMonitors::clear | 0x140949c40 | 78 | 显示器列表清理（O 名；与 0x140949c40 析构邻接，宿主疑为显示器缓存静态） | I | audio-misc\decomp\decomp-0x140949c40.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`default` 等 1 项
